use axum::{response::IntoResponse, Json};
use axum_extra::extract::SignedCookieJar;
use serde_json::json;
use crate::session::remove_session;

pub async fn logout(
    jar: SignedCookieJar,
) -> impl IntoResponse {
    let jar = remove_session(jar);

    // Sovrascrivo il vecchio cookie con uno vuoto
    (
        jar,
        Json(json!({
            "message": "User logged out successfully",
        })),
    )
}
