use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use zero2prod::{self, configuration::get_configuration};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let addr = test_app.address;
    let client = reqwest::Client::new();

    let response = client
        .get(format! {"{}/health_check",addr})
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn_app().await;
    let addr = test_app.address;

    let client = reqwest::Client::new();

    let msg_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(msg_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let test_app = spawn_app().await;
    let addr = test_app.address;

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_data, error_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_data)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 422 when the payload was {}.",
            error_msg
        );
    }
}

/// Starts an instance of our application
/// and returns its address (i.e. http://localhost:XXXX)
async fn spawn_app() -> TestApp {
    let db_pool = create_db_connection().await;
    // Binds to random port on local network
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server =
        zero2prod::startup::run(db_pool.clone(), listener).expect("Failed to bind address");
    tokio::spawn(server);
    // Return the address of server
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}

async fn create_db_connection() -> PgPool {
    let mut db_config = get_configuration()
        .expect("Failed to read configuration.")
        .database;
    db_config.database_name = Uuid::new_v4().to_string();

    // Create database
    let mut connection = PgConnection::connect(&db_config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&db_config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
