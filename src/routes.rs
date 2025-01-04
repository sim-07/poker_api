use axum::{routing::get, Router};
use serde_json::json;
use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { axum::Json(json!({ "message": "API HOME" })) }),
        )
        .route("/create_game", get(handlers::create_game::create_game))
}
