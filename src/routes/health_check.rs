use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    println!("Health Check!");
    HttpResponse::Ok().finish()
}
