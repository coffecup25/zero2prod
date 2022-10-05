use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};
use std::{future::Future, net::TcpListener};

use crate::routes::{health_check, subscriptions};

pub fn run(
    connection: Pool<Postgres>,
    listener: TcpListener,
) -> Result<impl Future<Output = hyper::Result<()>>, std::io::Error> {
    // let shared_state = Arc::new(State{connection});
    // build our application with a route
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions))
        .layer(Extension(connection));

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    let server = axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service());

    Ok(server)
}
