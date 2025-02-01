use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use redis::AsyncCommands;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    session::{add_session, read_session, SessionData},
    SharedState,
};

use crate::redis_client;

#[derive(serde::Deserialize, Debug)]
pub struct AddPlayerPayload {
    game_id: Uuid,
}

pub async fn join_game(
    State(shared_state): State<SharedState>,
    jar: SignedCookieJar,
    Json(payload): Json<AddPlayerPayload>,
) -> impl IntoResponse {
    println!("Payload received join_game: {:?}", payload);

    let user_id = match read_session(jar.clone()) {
        Some(session) => session.user_id,
        None => {
            return (jar, Json(json!({"error": "Error fetching user_id"})));
        }
    };

    let ex_data =
        redis_client::get_game_data(payload.game_id.to_string(), &Arc::new(shared_state.clone())).await;

    match ex_data {
        Ok(mut data) => {
            if let Some(players_str) = data.get_mut("players") {
                let mut players: Vec<String> = serde_json::from_str(players_str).unwrap_or_else(|_| vec![]);

                let new_player = user_id.unwrap().to_string();
                if !players.contains(&new_player) {
                    players.push(new_player);
                }

                *players_str = serde_json::to_string(&players).unwrap();
            } else {
                return (jar, Json(json!({"error": "Error adding player"})));
            }

            let shared_state = Arc::new(shared_state.clone());

            let mut con = match redis_client::redis_conn(&shared_state).await {
                Ok(con) => con,
                Err(_) => {
                    return (
                        jar,
                        Json(json!({"error": "Error fetching Redis connection"})),
                    )
                }
            };

            let game_key = payload.game_id.to_string();

            let data_tuples: Vec<(&str, &str)> =
                data.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();

            let _: () = con.hset_multiple(&game_key, &data_tuples).await.unwrap();

            let session_data = SessionData {
                game_id: Some(payload.game_id),
                user_id: user_id,
            };

            let jar = add_session(jar, session_data);

            (
                jar,
                Json(json!({
                    "message": "User added successfully",
                })),
            )
        }
        Err(e) => {
            return (
                jar,
                Json(json!({"error": format!("Error fetching data: {}", e)})),
            )
        }
    }
}
