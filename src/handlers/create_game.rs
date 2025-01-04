use axum::{Json, response::IntoResponse};
use reqwest::Client;
use serde_json::json;
use std::env;
use dotenvy::dotenv;
use uuid::Uuid;

#[derive(serde::Serialize)]
struct NewGame {
    id: Uuid,
    max_players: i32,
}

pub async fn create_game() -> impl IntoResponse {

    println!("create game called");

    dotenv().ok();

    let supabase_url = env::var("NEXT_PUBLIC_SUPABASE_URL").expect("SUPABASE_URL missing");
    let supabase_key = env::var("NEXT_PUBLIC_SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY missing");
    let table_name = env::var("TABLE_NAME").expect("SUPABASE_TABLE missing");

    let client = Client::new();

    let new_game = NewGame {
        id: Uuid::new_v4(),
        max_players: 4, // TODO cambiare con un parametro passato
    };

    let url = format!("{}/rest/v1/{}", supabase_url, table_name);

    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&new_game)
        .send()
        .await;

    match res {
        Ok(response) if response.status().is_success() => {
            let game: serde_json::Value = response.json().await.unwrap();
            Json(json!({ "status": "success", "game": game })).into_response()
        }
        Ok(response) => {
            Json(json!({ "status": "error", "message": response.status().to_string() })).into_response()
        }
        Err(err) => {
            Json(json!({ "status": "error", "message": err.to_string() })).into_response()
        }
    }
}