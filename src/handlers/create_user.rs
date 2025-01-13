use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use serde_json::json;
use sqlx::query;
use uuid::Uuid;

use crate::session::{add_session, SessionData};
use crate::routes::AppState;

#[derive(serde::Serialize)]
pub struct NewUser {
    name: String,
    id: Uuid
}

#[derive(serde::Deserialize, Debug)]
pub struct PayloadCreateUser {
    name: String
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<PayloadCreateUser>,
    jar: SignedCookieJar
) -> impl IntoResponse {

    let id = Uuid::new_v4();

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

    let _ = query!(
        "INSERT INTO users (name, id) VALUES ($1, $2)",
        payload.name,
        id
    )
    .execute(&mut *transaction) 
    .await
    .unwrap();

    transaction.commit().await.unwrap();

    let new_user = NewUser {
        id: id,
        name: payload.name
    };

    let session_data = SessionData {
        game_id: None,
        user_id: Some(id),
    };
    let jar = add_session(jar, session_data);

    (
        jar,
        Json(json!({
            "message": "User created successfully",
            "user": new_user,
        })),
    )
}
