use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use std::net::TcpListener;
use crate::routes::health_check;
use crate::routes::subscribe;
use sqlx::PgConnection;
use std::sync::Arc;

pub fn run(
    listener: TcpListener,
    connection: PgConnection
) -> Result<Server, std::io::Error> {
    let connection = Arc::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .data(connection.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}