use actix_web::{App, HttpResponse, HttpServer, dev::Server, get, post, web};
use serde::Deserialize;
use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Data<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}
