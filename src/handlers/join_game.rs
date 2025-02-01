use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use serde_json::json;
use uuid::Uuid;

use crate::{
    SharedState,
    session::{add_session, read_session, SessionData},
};

use crate::redis;

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

    let shared_state = Arc::clone(&shared_state.into());
    let ex_data = redis::get_game_data(payload.game_id.to_string(), &shared_state).await;

    match ex_data {
        Ok(ex_data) => {

            // TODO aggiustare. Conversione in json dà problemi
            let mut ex_data_json = serde_json::to_value(ex_data).unwrap();

            if let Some(cards_released) = ex_data_json.get_mut("cards_released") {
                if cards_released == &serde_json::json!("[]") {
                    *cards_released = serde_json::json!([]);
                }
            }

            println!("ex_data_json: {:?}", ex_data_json);
            let mut game_data: redis::GameData = serde_json::from_value(ex_data_json).unwrap();

            game_data.players.push(user_id.unwrap().to_string());

            println!("game_data: {:?}", game_data);

            let _ = redis::handle_game(&game_data, &shared_state).await;

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

        },
        Err(e) => return (jar, Json(json!({"error": format!("Error fetching data: {}", e)})))
    }
    
}
