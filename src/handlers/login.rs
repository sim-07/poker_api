use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use bcrypt::verify;
use serde_json::json;
use sqlx::query;
use uuid::Uuid;

use crate::routes::AppState;
use crate::session::{add_session, SessionData};

#[derive(serde::Deserialize, Debug)]
pub struct PayloadLogin {
    name: String,
    pass: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Json(payload): Json<PayloadLogin>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();

    let mut conn = state.db_pool.acquire().await.unwrap();

    let _ = match query!("SELECT pass FROM users WHERE name = $1", payload.name)
        .fetch_one(&mut *conn)
        .await
    {
        Ok(record) => {
            if let Some(hash_pass) = record.pass.as_deref() {
                match verify(&payload.pass, hash_pass) {
                    Ok(true) => {
                        let session_data = SessionData {
                            game_id: None,
                            user_id: Some(id),
                        };
                        let jar = add_session(jar, session_data);
                        
                        return (jar, Json(json!({"message": "User logged successfully"})));
                    }
                    Ok(false) => {
                        return (jar, Json(json!({"error": "Incorrect credentials"})));
                    }
                    Err(_) => {
                        return (jar, Json(json!({"error": "Error during login"})));
                    }
                }
            } else {
                return (jar, Json(json!({"error": "Error during login"})));
            }
        }
        Err(_) => {
            return (jar, Json(json!({"error": "Incorrect credentials"})));
        }
    };
}
