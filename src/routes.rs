use axum::{routing::{get, post}, Router};
use serde_json::json;
use axum_extra::extract::cookie::Key;
use axum::extract::FromRef;

use crate::ws;
use crate::handlers::{self};
use crate::SharedState;

impl FromRef<SharedState> for Key { // Per dire a Rust come ricavare key
    fn from_ref(state: &SharedState) -> Self {
        state.cookie_key.clone()
    }
}

pub fn create_routes(shared_state: SharedState) -> Router {

    Router::new()
        .route(
            "/",
            get(|| async { axum::Json(json!({ "message": "API HOME" })) }),
        )
        .route("/create_game", post(handlers::handlers_game::create_game::create_game))
        .route("/join_game", post(handlers::handlers_game::join_game::join_game))
        .route("/create_user", post(handlers::handlers_user::create_user::create_user))
        .route("/get_user_info", post(handlers::handlers_user::get_user_info::get_user_info))
        .route("/login", post(handlers::handlers_user::login::login))
        .route("/logout", post(handlers::handlers_user::logout::logout))
        .route("/ws", get(ws::handle_ws))
        .with_state(shared_state)
}