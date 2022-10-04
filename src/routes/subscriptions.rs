use axum::response::IntoResponse;
use hyper::StatusCode;
use serde::Deserialize;

pub async fn subscriptions(SignUp: axum::Form<SignUp>) -> impl IntoResponse {
    StatusCode::OK
}

#[derive(Deserialize)]
pub struct SignUp {
    email: String,
    name: String,
}
