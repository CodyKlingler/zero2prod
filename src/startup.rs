use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> Result<Server, std::io::Error> {
    // Wrap in Arc so that each thread has access to connection
    let connection = web::Data::new(connection);

    Ok(
        HttpServer::new(move || 
            App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
            )
            .listen(listener)?
            .run()
    )
}