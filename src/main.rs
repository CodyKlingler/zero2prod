use std::net::TcpListener;
use sqlx::PgPool;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let configuration = get_configuration().expect("failed to get configuration");

    let db_address = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&db_address)
        .await
        .expect("Failed to connect to database");

    let app_address = format!("127.0.0.1:{}", configuration.app_port );
 
    let listener = TcpListener::bind(app_address)?;

    run(listener, connection_pool)?.await
}
