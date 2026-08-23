use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}
