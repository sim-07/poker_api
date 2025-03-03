use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::SignedCookieJar;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{handlers::handlers_game::poker_actions::{call, fold, raise}, session::read_session};

#[derive(Debug, Deserialize)]
struct GameAction {
    //user_id: Uuid,
    action: String,
    amount: Option<u32>,
}

#[derive(Debug, Serialize)]
struct ServerResponse {
    status: String,
    message: String,
}

pub async fn handle_ws(ws: WebSocketUpgrade, jar: SignedCookieJar) -> Response {
    println!("jar in ws: {:?}", jar);
    let user_id = match read_session(jar.clone()) {
        Some(session) => match session.user_id {
            Some(id) => id,
            None => {
                println!("user_id is missing in session");
                return Json(json!({"error": "user_id is missing in session"})).into_response();
            }
        },
        None => {
            println!("Error fetching user_id in ws");
            return Json(json!({"error": "Error fetching user_id in ws"})).into_response();
        }
    };

    //println!("user_id in handle ws: {}", user_id);

    //let user_id= Uuid::new_v4();

    ws.on_upgrade(move |socket| handle_socket(socket, user_id))
}

async fn handle_socket(mut ws: WebSocket, user_id: Uuid) {
    while let Some(msg) = ws.recv().await {
        let msg = match msg {
            Ok(Message::Text(text)) => text,
            Ok(_) => continue,
            Err(_) => return, // Client disconnesso
        };

        let action: GameAction = match serde_json::from_str(&msg) {
            Ok(parsed) => parsed,
            Err(_) => {
                let error_response = ServerResponse {
                    status: "error".to_string(),
                    message: "Invalid message format".to_string(),
                };
                let _ = ws
                    .send(Message::Text(
                        serde_json::to_string(&error_response).unwrap().into(),
                    ))
                    .await;
                continue;
            }
        };

        match action.action.as_str() {
            "fold" => {
                let message = fold(user_id);
                let response = ServerResponse {
                    status: "success".to_string(),
                    message,
                };
                let _ = ws
                    .send(Message::Text(
                        serde_json::to_string(&response).unwrap().into(),
                    ))
                    .await;
            }
            "call" => {
                let message = call(user_id);
                let response = ServerResponse {
                    status: "success".to_string(),
                    message,
                };
                let _ = ws
                    .send(Message::Text(
                        serde_json::to_string(&response).unwrap().into(),
                    ))
                    .await;
            }
            "raise" => {
                if let Some(amount) = action.amount {
                    let message = raise(user_id, amount);
                    let response = ServerResponse {
                        status: "success".to_string(),
                        message,
                    };
                    let _ = ws
                        .send(Message::Text(
                            serde_json::to_string(&response).unwrap().into(),
                        ))
                        .await;
                } else {
                    let error_response = ServerResponse {
                        status: "error".to_string(),
                        message: "Error during raise".to_string(),
                    };
                    let _ = ws
                        .send(Message::Text(
                            serde_json::to_string(&error_response).unwrap().into(),
                        ))
                        .await;
                }
                
            }
            _ => {
                let error_response = ServerResponse {
                    status: "error".to_string(),
                    message: "Unknown action".to_string(),
                };
                let _ = ws
                    .send(Message::Text(
                        serde_json::to_string(&error_response).unwrap().into(),
                    ))
                    .await;
            }
        }
    }
}
