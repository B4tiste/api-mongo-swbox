use std::collections::HashMap;

use axum::{Json, extract::{Query, State}, http::StatusCode, routing::{get, post}, body::Bytes, Router};
use serde_json::{Value, json};
use crate::{subs::services, state::AppState};

async fn new_sub(
    State(state): State<AppState>,
    body: Bytes
) -> Result<Json<Value>, (StatusCode, Json<String>)> {
    // Convert the body to a Value for easier handling
    let body: Value = match serde_json::from_slice(&body) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error parsing subscription data: {}", err);
            return Err((StatusCode::BAD_REQUEST, Json("Invalid JSON".into())));
        }
    };

    services::save_membership_in_db(
        state.get_memberships_collection(),
        body
    ).await.map_err(|err| {
        eprintln!("Error saving membership: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, Json("Error saving membership".into()))
    })?;

    Ok(Json(json!({ "status": "Subscription received" })))
}

pub fn subs_routes(app_state: AppState) -> axum::Router {
    let routes = Router::new()
        .route("/new_sub", post(new_sub))
        .with_state(app_state);

    Router::new().nest("/subs", routes)
}