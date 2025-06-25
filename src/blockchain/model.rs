use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Block {
    pub index: u128,
    pub data: String,
    pub hash: String,
    pub previous: String,
    pub nonce: u64,
    pub timestamp: String
}