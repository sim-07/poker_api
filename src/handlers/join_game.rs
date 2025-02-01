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

    let ex_data = redis::get_game_data(payload.game_id.to_string(), shared_state.into()).await;

    match ex_data {
        Ok(ex_data) => {

            // TODO Creare game_data con i dati di ex_data, poi aggiungere l'user_id trovato nella sessione e sovrascrivere il tutto su redis

            let _ = redis::handle_game(&game_data, shared_state.into()).await;

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
        Err(_) => return (jar, Json(json!({"error": "Error fetching data"})))
    }
    
}
