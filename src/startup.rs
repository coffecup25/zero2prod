use axum::{
    routing::{get, post},
    Router,
};
use std::{future::Future, net::TcpListener};

use crate::routes::{health_check, subscriptions};

pub fn run(
    listener: TcpListener,
) -> Result<impl Future<Output = hyper::Result<()>>, std::io::Error> {
    // build our application with a route
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions));

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    let server = axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service());

    Ok(server)
}
