use sqlx::FromRow;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Actor {
    pub actor_id: u32,
    pub first_name: String,
}