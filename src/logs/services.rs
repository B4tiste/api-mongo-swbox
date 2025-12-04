use mongodb::{Collection, bson::doc};

use crate::logs::models::Log;

pub async fn get_nb_commands(
    logs: Collection<Log>,
    timestamp: Option<i64>
) -> Result<i64, mongodb::error::Error> {
    let filter = match timestamp {
        Some(ts) => doc! { "created_at": { "$gte": ts } },
        None => doc! {},
    };

    let count = logs.count_documents(filter).await?;
    Ok(count as i64)
}

pub async fn get_unique_users(
    logs: Collection<Log>,
    timestamp: Option<i64>
) -> Result<i64, mongodb::error::Error> {
    let filter = match timestamp {
        Some(ts) => doc! { "created_at": { "$gte": ts } },
        None => doc! {},
    };

    let distinct_users = logs.distinct("username", filter).await?;
    Ok(distinct_users.len() as i64)
}

pub async fn get_nb_servers(
    logs: Collection<Log>,
    timestamp: Option<i64>
) -> Result<i64, mongodb::error::Error> {
    let filter = match timestamp {
        Some(ts) => doc! { "created_at": { "$gte": ts } },
        None => doc! {},
    };

    let distinct_servers = logs.distinct("server_name", filter).await?;
    Ok(distinct_servers.len() as i64)
}