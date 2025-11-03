use mongodb::Collection;

use crate::logs::models::Log;

#[derive(Clone)]
pub struct AppState {
    pub mongo: mongodb::Client,
}

impl AppState {
    pub fn get_logs_collection(&self) -> Collection<Log> {
        self.mongo.database("bot-swbox-db").collection("logs")
    }
}