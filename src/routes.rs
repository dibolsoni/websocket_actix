use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::web::Path;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .body("Hello world!")
        .into()
}

pub async fn hello(_: HttpRequest, name: Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}
