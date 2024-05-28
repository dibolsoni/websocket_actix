use crate::lobby::Lobby;
use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::{ActorContext, ContextFutureSpawner, WrapFuture, ActorFutureExt};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use actix::fut::ready;
use uuid::Uuid;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(10);


pub struct WsConn {
    room_id: Uuid,
    lobby_address: Addr<Lobby>,
    heartbeat: Instant,
    id: Uuid,
}

impl WsConn {
    pub fn new(room: Uuid, lobby_address: Addr<Lobby>) -> Self {
        WsConn {
            room_id: room,
            lobby_address,
            heartbeat: Instant::now(),
            id: Uuid::new_v4(),
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We start heartbeat async process here.
    /// We'll also register self in Lobby.
    /// If register fails then we stop actor.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let address = ctx.address();
        self.lobby_address
            .send(Connect {
                address: address.recipient(),
                lobby_id: self.room_id,
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, _ctx| {
                match res {
                    Ok(_) => (),
                    _ => _ctx.stop(),
                }
                ready(())
            })
            .wait(ctx);
    }

    /// Method is called on actor stop.
    /// We'll unregister from Lobby.
    /// Stop actor sync.
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_address.do_send(Disconnect {
            id: self.id,
            room_id: self.room_id,
        });
        Running::Stop
    }
}

impl WsConn {
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > HEARTBEAT_TIMEOUT {
                act.lobby_address.do_send(Disconnect {
                    id: act.id,
                    room_id: act.room_id,
                });
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin)
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(s)) => {
                self.lobby_address.do_send(ClientActorMessage {
                    id: self.id,
                    message: s.parse().unwrap(),
                    room_id: self.room_id
                });
            }
            Err(e) => panic!("{}", e),
        }
    }
}


impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, message: WsMessage, ctx: &mut Self::Context) {
        ctx.text(message.0);
    }
}
