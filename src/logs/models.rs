use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub username: String,
    pub command_name: String,
    pub server_name: String,
    pub command_result: bool,
    pub created_at: i64
}