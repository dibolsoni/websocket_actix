use actix::prelude::{Message, Recipient};
use uuid::Uuid;

/// rtype is the same of Result in Handler trait

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);


#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Connect{
    pub address: Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect{
    pub id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub room_id: Uuid,
    pub message: String,
}
