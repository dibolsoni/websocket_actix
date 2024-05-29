use std::sync::Mutex;

use actix::{Actor, Addr, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, Responder, web::Data, web::Path, web::Payload};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::another_ws::AnotherWsConn;
use crate::lobby::Lobby;
use crate::ws::WsConn;

pub async fn start_ws(
    req: HttpRequest,
    stream: Payload,
    group_id: Path<Uuid>,
    lobby: Data<Addr<Lobby>>,
) -> impl Responder {
    let ws = WsConn::new(
        group_id.into_inner(),
        lobby,
    );
    ws::start(ws, &req, stream).unwrap()
}

pub async fn another_ws(
    req: HttpRequest,
    stream: Payload,
    id: Path<i32>,
    counter: Data<Mutex<i32>>
) -> Result<HttpResponse, Error> {
    ws::start(
        AnotherWsConn::new(counter, id.into_inner()),
        &req,
        stream
    )
}

pub struct TestWs {}

impl Actor for TestWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TestWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}
