use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let configuration = get_configuration().expect("failed to get configuration");

    let address = format!("127.0.0.1:{}", configuration.app_port );
 
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
