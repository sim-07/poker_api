use uuid::Uuid;


pub fn fold(user_id: Uuid) -> String {
    format!("Player {} folded", user_id)
}

pub fn raise(user_id: Uuid, amount: u32) -> String {
    format!("Player {} raise {}", user_id, amount)
}

pub fn call(user_id: Uuid) -> String {
    format!("Player {} call", user_id)
}