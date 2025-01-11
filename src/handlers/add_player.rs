use axum::{response::IntoResponse, Json};
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::json;
use sqlx::{query, PgPool};
use std::env;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct AddPlayer {
    game_id: Uuid,
    id_player: Uuid,
}

pub async fn add_player(Json(payload): Json<AddPlayer>) -> impl IntoResponse {
    println!("Payload received add_player: {:?}", payload);

    //let mut transaction = shared_pool.begin().await.unwrap();

    //let players = query!("SELECT players FROM games WHERE id = $1", payload.game_id)
       // .execute(&mut *transaction)
       // .await
        //.unwrap();
}
