use std::collections::HashSet;
use std::sync::Mutex;

use actix_web::{App, Error, HttpServer};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::{Data, resource};

use crate::lobby::Lobby;
use crate::routes::{hello, index};
use crate::start_connection::{another_ws, start_ws};

mod ws;
mod messages;
mod lobby;
mod start_connection;
mod test_ws;
mod routes;
mod another_ws;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Config=(),
        InitError=(),
        Error=Error
    >,
> {
    App::new()
        .service(resource("/ws/{group_id}").to(start_ws))
        .service(resource("/").to(index))
        .service(resource("/hello/{name}").to(hello))
        .service(resource("/another_ws/{id}").to(another_ws))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let counter = Data::new(Mutex::new(0i32));
    let lobby = Data::new(Lobby::default());
    let users: Data<HashSet<i32>> = Data::new(HashSet::new());
    HttpServer::new(move ||
        create_app()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Logger::new("%a %{User-Agent}i"))
            .app_data(lobby.clone())
            .app_data(users.clone())
            .app_data(counter.clone())
    )
        .bind("127.0.0.1:3030")
        .expect("Can not bind to port 3030")
        .run()
        .await
}
