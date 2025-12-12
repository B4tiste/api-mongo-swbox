use mongodb::Collection;

use crate::logs::models::Log;
use crate::subs::models::Membership;

#[derive(Clone)]
pub struct AppState {
    pub mongo: mongodb::Client,
}

impl AppState {
    pub fn get_logs_collection(&self) -> Collection<Log> {
        self.mongo.database("bot-swbox-db").collection("logs")
    }

    pub fn get_memberships_collection(&self) -> Collection<Membership> {
        self.mongo.database("bot-swbox-db").collection("memberships")
    }
}