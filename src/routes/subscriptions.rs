use axum::{response::IntoResponse, Extension};
use hyper::StatusCode;
use serde::Deserialize;

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn subscriptions(
    axum::Form(sign_up): axum::Form<SignUp>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        sign_up.email,
        sign_up.name,
        Utc::now()
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            println!("Failed to execute query: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Deserialize)]
pub struct SignUp {
    pub email: String,
    pub name: String,
}
