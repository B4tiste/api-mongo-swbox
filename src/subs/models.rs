use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Membership {
    pub transaction_id: String,
    pub email: String,
    pub claimed: bool,
    pub created_at: i64,
}