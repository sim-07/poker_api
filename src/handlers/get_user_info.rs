use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use serde_json::json;
use sqlx::query;
use uuid::Uuid;

use crate::routes::AppState;
use crate::session::read_session;

#[derive(serde::Serialize)]
struct Data {
    user_id: Uuid,
    name: String
}


pub async fn get_user_info(
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> impl IntoResponse {

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


    let data = match query!("SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&mut *conn)
        .await
    {
        Ok(record) => record,
        Err(_) => {
            return (jar, Json(json!({"error": "No result found"})));
        }
    };

    let data_fetched = Data {
        name: data.name,
        user_id: data.id
    };

    (
        jar,
        Json(json!({
            "message": "User fetched successfully",
            "data": data_fetched
        })),
    )


}
