use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io, sync::Arc};

use crate::SharedState;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameData {
    pub game_id: String,
    pub players: Vec<String>,
    pub pot: u32,
    pub round: u32,
    pub cards_released: Vec<String>,
    pub initial_fiches: u32,
    pub small_blind: u32,
}

async fn redis_conn(
    shared_state: &Arc<SharedState>,
) -> Result<
    bb8_redis::bb8::PooledConnection<'_, bb8_redis::RedisConnectionManager>,
    redis::RedisError,
> {
    shared_state.redis_pool.get().await.map_err(|e| {
        let io_error = io::Error::new(io::ErrorKind::Other, e.to_string());
        redis::RedisError::from(io_error)
    })
}

pub async fn handle_game(
    game_data: &GameData,
    shared_state: &Arc<SharedState>,
) -> Result<(), redis::RedisError> {
    let mut con = redis_conn(&shared_state).await?;

    let _: () = con
        .hset_multiple(
            &game_data.game_id,
            &[
                ("players", serde_json::to_string(&game_data.players).unwrap()),
                ("pot", game_data.pot.to_string()),
                ("round", game_data.round.to_string()),
                ("cards_released", serde_json::to_string(&game_data.cards_released).unwrap()),
                ("initial_fiches", game_data.initial_fiches.to_string()),
                ("small_blind", game_data.small_blind.to_string()),
            ],
        )
        .await?;

    Ok(())
}

pub async fn get_game_data(
    game_id: String,
    shared_state: &Arc<SharedState>,
) -> Result<HashMap<String, String>, String> {
    let mut con = match redis_conn(&shared_state).await {
        Ok(con) => con,
        Err(_) => return Err("Error fetching connection".to_string()),
    };

    let res: Result<HashMap<String, String>, redis::RedisError> = con.hgetall(game_id).await;

    match res {
        Ok(data) => {
            if data.is_empty() {
                Err("No data found for gameid".to_string())
            } else {
                Ok(data)
            }
        }
        Err(_) => Err("Error fetching data".to_string()),
    }
}
