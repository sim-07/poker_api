use axum::{routing::{get, post}, Router};
use serde_json::json;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::handlers::{self, create_user};

pub fn create_routes(db_pool: Pool<Postgres>) -> Router {

    let shared_pool = Arc::new(db_pool);

    Router::new()
        .route(
            "/",
            get(|| async { axum::Json(json!({ "message": "API HOME" })) }),
        )
        .route("/create_game", post(handlers::create_game::create_game))
        .route("/add_player", post(handlers::add_player::add_player))
        .route("/create_user", post(handlers::create_user::create_user))
        .with_state(shared_pool)
}
