use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use serde_json::json;
use std::vec;
use uuid::Uuid;

use crate::redis;
use crate::{
    session::{add_session, read_session, SessionData},
    SharedState,
};

#[derive(serde::Deserialize, Debug)]
pub struct PayloadCreateGame {
    initial_fiches: u32,
    small_blind: u32,
}

pub async fn create_game(
    State(shared_state): State<SharedState>,
    jar: SignedCookieJar,
    Json(payload): Json<PayloadCreateGame>,
) -> impl IntoResponse {
    let game_id = Uuid::new_v4();

    let user_id = match read_session(jar.clone()) {
        Some(session) => session.user_id,
        None => {
            return (jar, Json(json!({"error": "Error fetching user_id"})));
        }
    };

    let game_data = redis::GameData {
        game_id: game_id.to_string(),
        players: vec![user_id.unwrap().to_string()],
        pot: 0,
        round: 0,
        cards_released: vec![],
        initial_fiches: payload.initial_fiches,
        small_blind: payload.small_blind,
    };

    let _ = redis::handle_game(&game_data, shared_state.into()).await;

    let session_data = SessionData {
        game_id: Some(game_id),
        user_id: user_id,
    };
    let jar = add_session(jar, session_data);

    (
        jar,
        Json(json!({
            "message": "Game created successfully",
            "user": game_data,
        })),
    )
}
