use axum::{response::IntoResponse, Json};
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::json;
use std::env;
use uuid::Uuid;

#[derive(serde::Serialize)]
struct NewGame {
    id: Uuid,
    max_players: i32,
}

pub async fn create_game() -> impl IntoResponse {
    println!("create_game endpoint called");

    dotenv().ok();

    let supabase_url = env::var("NEXT_PUBLIC_SUPABASE_URL").expect("SUPABASE_URL missing");
    let supabase_key =
        env::var("NEXT_PUBLIC_SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY missing");
    let table_name = env::var("TABLE_NAME").expect("SUPABASE_TABLE missing");

    println!(
        "Environment variables loaded:\n  - SUPABASE_URL: {}\n  - TABLE_NAME: {} - ANON KEY: {}",
        supabase_url, table_name, supabase_key
    );

    let client = Client::new();

    let new_game = NewGame {
        id: Uuid::new_v4(),
        max_players: 4, // TODO cambiare con un parametro passato
    };

    println!("New game created with ID: {}", new_game.id);

    let url = format!("{}/rest/v1/{}", supabase_url, table_name);

    println!("Sending POST request to URL: {}", url);

    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("apikey", supabase_key)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&new_game)
        .send()
        .await;

    println!("Received response: {:?}", res);

    match res {
        Ok(response) if response.status().is_success() => {
            println!("Request succeeded with status: {}", response.status());
            let game: serde_json::Value = response.json().await.unwrap();
            println!("Response body: {:?}", game);
            Json(json!({ "status": "success", "game": game })).into_response()
        }
        Ok(response) => {
            println!("Request failed with status: {}", response.status());
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "No response body".to_string());
            println!("Error response body: {}", error_message);
            Json(json!({ "status": "error", "message": error_message })).into_response()
        }
        Err(err) => {
            println!("Request failed with error: {}", err);
            Json(json!({ "status": "error", "message": err.to_string() })).into_response()
        }
    }
}
