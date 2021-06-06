use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(_from: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}