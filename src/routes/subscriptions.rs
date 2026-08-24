use actix_web::{HttpResponse, post, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Data<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
