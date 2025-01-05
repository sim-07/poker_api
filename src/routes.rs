use axum::{routing::{get, post}, Router};
use serde_json::json;
use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { axum::Json(json!({ "message": "API HOME" })) }),
        )
        .route("/create_game", post(handlers::create_game::create_game))
        .route("/add_player", post(handlers::add_player::add_player))
}
