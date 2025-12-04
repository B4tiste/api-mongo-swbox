use std::collections::HashMap;

use axum::{Json, extract::{Query, State}, http::StatusCode, routing::get, Router};
use serde_json::{Value, json};
use crate::{logs::services, state::AppState};

async fn get_nb_commands(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, (StatusCode, Json<String>)> {
    let timestamp = params.get("timestamp").and_then(|ts| ts.parse().ok());

    match services::get_nb_commands(state.get_logs_collection(), timestamp).await {
        Ok(count) => Ok(Json(json!({ "nb": count }))),
        Err(err) => {
            eprintln!("Error getting command count: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Error".into())))
        }
    }
}

async fn get_unique_users(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, (StatusCode, Json<String>)> {
    let timestamp = params.get("timestamp").and_then(|ts| ts.parse().ok());

    match services::get_unique_users(state.get_logs_collection(), timestamp).await {
        Ok(count) => Ok(Json(json!({ "nb": count }))),
        Err(err) => {
            eprintln!("Error getting unique users: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Error".into())))
        }
    }
}

async fn get_nb_servers(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, (StatusCode, Json<String>)> {
    let timestamp = params.get("timestamp").and_then(|ts| ts.parse().ok());

    match services::get_nb_servers(state.get_logs_collection(), timestamp).await {
        Ok(count) => Ok(Json(json!({ "nb": count }))),
        Err(err) => {
            eprintln!("Error getting servers count: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Error".into())))
        }
    }
}

pub fn logs_routes(app_state: AppState) -> axum::Router {
    let routes = Router::new()
        .route("/commands", get(get_nb_commands))
        .route("/users", get(get_unique_users))
        .route("/servers", get(get_nb_servers))
        .with_state(app_state);

    Router::new().nest("/logs", routes)
}