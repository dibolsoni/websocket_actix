use std::collections::{HashMap, HashSet};
use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;
use crate::messages::{WsMessage, Connect, Disconnect, ClientActorMessage};

type Socket = Recipient<WsMessage>;


#[derive(Clone)]
pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    rooms: HashMap<Uuid, HashSet<Uuid>>,
}


impl Default for Lobby {
    fn default() -> Self {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_message(&self, to_id: Uuid, message: &str) {
        if let Some(socket_recipient) = self.sessions.get(&to_id) {
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));
        } else {
            println!("Not found user: [{}] on sending message", to_id);
        }
    }
}


impl Actor for Lobby {
    type Context = Context<Self>;
}


impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|conn_id| {
                    self.send_message(
                        conn_id.to_owned(),
                        &format!("User: [{}] has left the room", msg.id)
                    );
                });

            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}


impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        self.rooms.entry(msg.lobby_id).or_insert_with(HashSet::new).insert(msg.self_id);

        self.rooms.get(&msg.lobby_id).unwrap().iter().for_each(|conn_id| {
            self.send_message(
                conn_id.to_owned(),
                &format!("User: [{}] has joined the room", msg.self_id)
            );
        });

        self.sessions.insert(
            msg.self_id,
            msg.address,
        );

        self.send_message(msg.self_id, &format!("Your id is {}", msg.self_id));
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Self::Context) -> Self::Result {
        if msg.message.starts_with("\\w") {
            if let Some(to_id) = msg.message.split_whitespace().nth(1) {
                if let Ok(to_id) = to_id.parse::<Uuid>() {
                    self.send_message(
                        to_id,
                        &format!("Whisper from [{}]: {}", msg.id, msg.message)
                    );
                }
            }
        } else {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .for_each(|conn_id| {
                    self.send_message(
                        conn_id.to_owned(),
                        &format!("[{}]: {}", msg.id, msg.message)
                    );
                });
        }
    }
}
