use bb8_redis::bb8::Pool;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::SharedState;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameData {
    pub game_id: String,
    pub players: Vec<String>,
    pub pot: u64,
    pub round: u32,
    pub cards_released: Vec<String>,
}

pub async fn create_game(
    game_data: &GameData,
    shared_state: Arc<SharedState>,
) -> Result<(), redis::RedisError> {
    
    let mut con = shared_state
        .redis_pool
        .get()
        .await
        .map_err(|e| redis::RedisError::from((redis::ErrorKind::IoError, e.to_string())))?;

    let _: () = con
        .hset_multiple(
            &game_data.game_id,
            &[
                (
                    "players",
                    serde_json::to_string(&game_data.players).unwrap(),
                ),
                ("pot", game_data.pot.to_string()),
                ("turno", game_data.round.to_string()),
                (
                    "carte_uscite",
                    serde_json::to_string(&game_data.cards_released).unwrap(),
                ),
            ],
        )
        .await?;

    Ok(())
}

pub async fn update_game(
    game_data: &GameData,
    shared_state: Arc<SharedState>,
) -> Result<(), redis::RedisError> {
    //TODO

    Ok(())
}
