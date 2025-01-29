use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use bcrypt::{hash, DEFAULT_COST};
use serde_json::json;
use sqlx::query;
use uuid::Uuid;

use crate::SharedState;
use crate::session::{add_session, SessionData};

#[derive(serde::Deserialize, Debug)]
pub struct PayloadCreateUser {
    name: String,
    pass: String,
}

pub async fn create_user(
    State(state): State<SharedState>,
    jar: SignedCookieJar,
    Json(payload): Json<PayloadCreateUser>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();

    let mut conn = state.db_pool.acquire().await.unwrap();

    let _ = match query!("SELECT * FROM users WHERE name = $1", payload.name)
        .fetch_optional(&mut *conn)
        .await
    {
        ///// Utente già esistente /////
        Ok(Some(_)) => {
            return (jar, Json(json!({"error" : "User already exist"})));
        }

        ///// Nessun utente con lo stesso nome /////
        Ok(None) => {

            let hashed_pass = hash(&payload.pass, DEFAULT_COST).unwrap();

            let _ = query!(
                "INSERT INTO users (name, pass, id) VALUES ($1, $2, $3)",
                payload.name,
                hashed_pass,
                id,
            )
            .execute(&mut *conn)
            .await
            .unwrap();

        }

        ///// Errore nel db /////
        Err(_) => {
            return (jar, Json(json!({"error" : "Db error"})));
        }
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
        })),
    )
}
