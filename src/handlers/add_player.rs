// AGGIUNGERE UN PLAYER AL JSON NEL DB, ORA SOVRASCRIVE SOLTANTO



use axum::{response::IntoResponse, Json};
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::json;
use std::env;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct AddPlayer {
    game_id: Uuid,
    id_player: Uuid,
}

pub async fn add_player(Json(payload): Json<AddPlayer>) -> impl IntoResponse {
    println!("Payload received add_player: {:?}", payload);

    let client = Client::new();

    dotenv().ok();

    let supabase_url = env::var("NEXT_PUBLIC_SUPABASE_URL").expect("SUPABASE_URL missing");
    let supabase_key =
        env::var("NEXT_PUBLIC_SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY missing");
    let table_name = env::var("TABLE_NAME").expect("SUPABASE_TABLE missing");

    let url = format!(
        "{}/rest/v1/{}?game_id=eq.{}",
        supabase_url, table_name, payload.game_id
    ); // ?game_id=eq. per aggiornare i valori in base a game_id

    let player = json!({
        "id_player": payload.id_player
    });

    let res = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("apikey", supabase_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&player)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("Player added successfully.");
                Json(json!({ "message": "Player added successfully." })).into_response()
            } else {
                println!(
                    "Failed to add player: {:?}",
                    response.text().await.unwrap()
                );
                Json(json!({ "error": "Failed to add player." })).into_response()
            }
        }
        Err(e) => {
            println!("Request error: {:?}", e);
            Json(json!({ "error": "Internal server error." })).into_response()
        }
    }
}