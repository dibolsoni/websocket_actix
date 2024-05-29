use actix::prelude::{Message, Recipient};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// rtype is the same of Result in Handler trait

#[derive(Message, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub user_id: Uuid,
    pub message: String,
}

impl WsMessage {
    pub fn new(user_id: Uuid, message: String) -> Self {
        WsMessage {
            user_id,
            message,
        }
    }
}




#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Connect{
    pub address: Recipient<WsMessage>,
    pub room_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect{
    pub user_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub message: String,
}
