use std::net::TcpListener;

use sqlx::{Pool, Postgres};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool: Pool<Postgres> =
        Pool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).unwrap();
    run(connection_pool, listener).unwrap().await.unwrap();
}
