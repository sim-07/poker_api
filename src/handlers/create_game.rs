use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use serde_json::json;
use sqlx::query;
use std::vec;
use uuid::Uuid;

use crate::{routes::AppState, session::{add_session, read_session, SessionData}};


#[derive(serde::Serialize)]
struct NewGame {
    id: Uuid,
    max_players: i32,
    fill_with_bot: bool,
    show_value_hand: bool,
    initial_fiches: i32,
    small_blind: i32,
    players: Vec<Uuid>,
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
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Json(payload): Json<PayloadCreateGame>,
) -> impl IntoResponse {
    let game_id = Uuid::new_v4();

    let mut conn = state.db_pool.acquire().await.unwrap();


    let user_id = match read_session(jar.clone()) {
        Some(session) => session.user_id,
        None => {
            return (
            jar,
            Json(json!({"error": "Error fetching user_id"})),
            );
        }
    };

    let _ = query!(
        "INSERT INTO games (game_id, max_players, fill_with_bot, show_value_hand, initial_fiches, small_blind, players) 
        VALUES ($1, $2, $3, $4, $5, $6, $7)",
        game_id,
        payload.max_players,
        payload.fill_with_bot,
        payload.show_value_hand,
        payload.initial_fiches as i64,
        payload.small_blind as i64,
        &vec![user_id.unwrap()]
    )
    .execute(&mut *conn) 
    .await
    .unwrap();

    let new_game = NewGame {
        id: game_id,
        max_players: payload.max_players,
        fill_with_bot: payload.fill_with_bot,
        show_value_hand: payload.show_value_hand,
        initial_fiches: payload.initial_fiches,
        small_blind: payload.small_blind,
        players: vec![user_id.unwrap()]
    };

    let session_data = SessionData {
        game_id: Some(game_id),
        user_id: user_id,
    };
    let jar = add_session(jar, session_data);

    (
        jar,
        Json(json!({
            "message": "User created successfully",
            "user": new_game,
        })),
    )
}
