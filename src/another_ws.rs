use std::sync::Mutex;

use actix::{Actor, StreamHandler};
use actix_web::web::Data;
use actix_web_actors::ws;

pub struct AnotherWsConn {
    id: i32,
    counter: Data<Mutex<i32>>,
}


impl AnotherWsConn {
    pub fn new(counter: Data<Mutex<i32>>, id: i32) -> Self {
        AnotherWsConn {
            id,
            counter,
        }
    }
}

impl Actor for AnotherWsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text(format!("AnotherWsConn started: {}", self.id))
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AnotherWsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Text(text)) => {
                let mut counter = self.counter.lock().unwrap();
                *counter += 1;
                ctx.text(format!(
                    "counter: {:?} - msg: {}", counter, text))
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}
