use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use serde_json::json;
use sqlx::query;
use uuid::Uuid;

use crate::{
    routes::AppState,
    session::{add_session, read_session, SessionData},
};

#[derive(serde::Deserialize, Debug)]
pub struct AddPlayerPayload {
    game_id: Uuid,
}

pub async fn add_player(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Json(payload): Json<AddPlayerPayload>,
) -> impl IntoResponse {
    println!("Payload received add_player: {:?}", payload);

    let user_id = match read_session(jar.clone()) {
        Some(session) => session.user_id,
        None => {
            return (jar, Json(json!({"error": "Error fetching user_id"})));
        }
    };

    let mut transaction = match state.db_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            eprintln!("Errore nell'avvio della transazione: {:?}", err);
            return (
                jar,
                Json(json!({ "error": "Errore durante l'avvio della transazione" })),
            );
        }
    };

    let result = query!(
        "UPDATE games
        SET players = 
            CASE
                WHEN NOT ($1 = ANY(players)) THEN array_append(players, $1)
                ELSE players
            END
        WHERE game_id = $2;",
        user_id,
        payload.game_id
    )
    .execute(&mut *transaction)
    .await;

    match result {
        Ok(_) => {}
        Err(_) => {
            return (
                jar,
                Json(json!({
                    "error": "No game found",
                })),
            );
        }
    }

    transaction.commit().await.unwrap();

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
