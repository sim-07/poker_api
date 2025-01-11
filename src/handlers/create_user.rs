use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use sqlx::{query, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct NewUser {
    name: String,
    id: Uuid
}

#[derive(serde::Deserialize, Debug)]
pub struct PayloadCreateUser {
    name: String
}

pub async fn create_user(
    State(shared_pool): State<Arc<PgPool>>,
    Json(payload): Json<PayloadCreateUser>,
) -> impl IntoResponse {

    let id = Uuid::new_v4();

    let mut transaction = shared_pool.begin().await.unwrap();

    let _ = query!(
        "INSERT INTO users (name, id) VALUES ($1, $2)",
        payload.name,
        id
    )
    .execute(&mut *transaction) 
    .await
    .unwrap();

    transaction.commit().await.unwrap();

    let new_user = NewUser {
        id: id,
        name: payload.name
    };

    Json(json!({
        "message": "User created successfully",
        "user": new_user,
    }))
}
