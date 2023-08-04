use std::net::TcpListener;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use sqlx::{PgConnection, Connection, PgPool, Executor};
use uuid::Uuid;


struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> TestApp {
    let app_listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let mut configuration = get_configuration().expect("Failed to read configuration file");

    // change name to arbitrary string
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&configuration.database).await;

    let app_port = app_listener.local_addr().unwrap().port();

    let app_server = zero2prod::startup::run(app_listener, db_pool.clone()).expect("Failed to bind to address");

    let _ = tokio::spawn(app_server);

    let address = format!("http://127.0.0.1:{app_port}");

    TestApp{ address, db_pool }
}

// creates database, migrates it, returns a connection pool to it.
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let postgres_addr = config.connection_string_without_db();

    let mut connection = PgConnection::connect(&postgres_addr)
        .await
        .expect("Failed to connect to Postgres");

   
    connection.execute(format!(r#"CREATE DATABASE"{}";"#, config.database_name).as_str())
    .await
    .expect("Failed to create database");

    let pool = PgPool::connect(&config.connection_string())
    .await
    .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Failed to migrate database");

    pool
}


// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let test_data = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &test_data.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    
    let test_data = &spawn_app().await;

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    
    let response = client
        .post(&format!("{}/subscriptions", &test_data.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_data.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let test_data = &spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
        ("name=&email=abc@gmail.com", "empty name"),
        ("name=madame&email=", "empty email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &test_data.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}