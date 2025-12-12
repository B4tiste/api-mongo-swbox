use mongodb::{Collection, bson::doc};

use serde_json::Value;
use crate::subs::models::Membership;

pub async fn save_membership_in_db(
    memberships: Collection<Membership>,
    body: Value
) -> Result<(), mongodb::error::Error> {
    // Extract relevant fields from the body
    let transaction_id = body.get("kofi_transaction_id")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let email = body.get("email")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let created_at = body.get("timestamp")
        .and_then(|v| v.as_str())
        .and_then(|ts| chrono::DateTime::parse_from_rfc3339(ts).ok())
        .map(|dt| dt.timestamp())
        .unwrap_or_else(|| chrono::Utc::now().timestamp());

    let membership = Membership {
        transaction_id,
        email,
        claimed: false,
        created_at,
    };
    println!("Membership to save: {:?}", membership);
    memberships.insert_one(membership).await?;
    Ok(())
}