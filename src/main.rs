use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello World!\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
