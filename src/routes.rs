use axum::{routing::{get, post}, Router};
use serde_json::json;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use axum_extra::extract::cookie::Key;
use axum::extract::FromRef;

use crate::handlers::{self};

impl FromRef<AppState> for Key { // Per dire a Rust come ricavare key
    fn from_ref(state: &AppState) -> Self {
        (*state.key).clone()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<Pool<Postgres>>,
    pub key: Arc<Key>,
}

pub fn create_routes(db_pool: Pool<Postgres>, shared_state: Arc<Key>) -> Router {

    let app_state = AppState {
        db_pool: Arc::new(db_pool),
        key: shared_state,
    };  

    Router::new()
        .route(
            "/",
            get(|| async { axum::Json(json!({ "message": "API HOME" })) }),
        )
        .route("/create_game", post(handlers::create_game::create_game))
        .route("/create_user", post(handlers::create_user::create_user))
        .route("/add_player", post(handlers::add_player::add_player))
        .with_state(app_state)
}