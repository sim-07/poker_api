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
    fill_with_bot: bool,
    show_value_hand: bool,
    initial_fiches: i32,
    small_blind: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct PayloadCreateGame {
    max_players: i32,
    fill_with_bot: bool,
    show_value_hand: bool,
    initial_fiches: i32,
    small_blind: i32,
}

pub async fn create_game(Json(payload): Json<PayloadCreateGame>) -> impl IntoResponse {
    println!("create_game endpoint called");
    println!("Payload received create_game: {:?}", payload);

    dotenv().ok();

    let supabase_url = env::var("NEXT_PUBLIC_SUPABASE_URL").expect("SUPABASE_URL missing");
    let supabase_key =
        env::var("NEXT_PUBLIC_SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY missing");
    let table_name = env::var("TABLE_NAME").expect("SUPABASE_TABLE missing");

    let client = Client::new();

    if payload.max_players < 2 || payload.max_players > 30 {
        return Json(json!({ "status": "error", "message": "max_players must be between 2 and 30" })).into_response();
    }
    
    if payload.initial_fiches <= 0 {
        return Json(json!({ "status": "error", "message": "initial_fiches must be greater than 0" })).into_response();
    }
    
    if payload.small_blind <= 0 || payload.small_blind > payload.initial_fiches {
        return Json(json!({ "status": "error", "message": "small_blind must be greater than 0 and less than initial_fiches" })).into_response();
    }

    let new_game = NewGame {
        id: Uuid::new_v4(),
        max_players: payload.max_players,
        fill_with_bot: payload.fill_with_bot,
        show_value_hand: payload.show_value_hand,
        initial_fiches: payload.initial_fiches,
        small_blind: payload.small_blind,
    };

    println!("New game created with ID: {}", new_game.id);

    let url = format!("{}/rest/v1/{}", supabase_url, table_name);

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
            Json(json!({ "status": response.status().to_string() })).into_response()
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
