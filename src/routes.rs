use axum::{routing::get, Router};
use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { axum::Json(json!({ "message": "Benvenuto nell'API!" })) }),
        )
        .route("/create_game", get(handlers::create_game))
}
