use axum_extra::extract::cookie::Key;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;

pub async fn handle_ws(
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut ws: WebSocket) {
    while let Some(msg) = ws.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return; // client disconnected
        };
        if ws.send(msg).await.is_err() {
            return; // client disconnected
        }
    }
}