use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use sqlx::{query, PgPool};
use std::sync::Arc;
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

pub async fn create_game(
    State(shared_pool): State<Arc<PgPool>>,
    Json(payload): Json<PayloadCreateGame>,
) -> impl IntoResponse {
    let game_id = Uuid::new_v4();

    let mut transaction = shared_pool.begin().await.unwrap();


    let _ = query!(
        "INSERT INTO games (id, max_players, fill_with_bot, show_value_hand, initial_fiches, small_blind) 
        VALUES ($1, $2, $3, $4, $5, $6)",
        game_id,
        payload.max_players,
        payload.fill_with_bot,
        payload.show_value_hand,
        payload.initial_fiches as i64,
        payload.small_blind as i64
    )
    .execute(&mut *transaction) 
    .await
    .unwrap();

    transaction.commit().await.unwrap();

    let new_game = NewGame {
        id: game_id,
        max_players: payload.max_players,
        fill_with_bot: payload.fill_with_bot,
        show_value_hand: payload.show_value_hand,
        initial_fiches: payload.initial_fiches,
        small_blind: payload.small_blind,
    };

    Json(json!({
        "message": "Game created successfully",
        "game": new_game,
    }))
}
