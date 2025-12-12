use axum::{Extension, Router};
use tower_http::cors::{Any, CorsLayer};
use shuttle_runtime::SecretStore;

use api_mongo_swbox::{state::AppState, routes::{logs, subs}};

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let mongo_uri = secret_store.get("MONGO_URI").expect("missing mongo_uri");
    let mongo = mongodb::Client::with_uri_str(&mongo_uri).await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin([
            "https://bot-swbox.netlify.app".parse().unwrap(),   // Website
            "http://localhost:4200".parse().unwrap(),           // Local website dev
            "http://127.0.0.1:4200".parse().unwrap(),           // Local website dev
            "http://127.0.0.1:8000".parse().unwrap(),           // Local API dev
        ]).allow_methods(Any).allow_headers(Any);
    let app_state = AppState { mongo: mongo };

    // Routers
    let logs_routes = logs::logs_routes(app_state.clone());
    let subs_routes = subs::subs_routes(app_state.clone());

    let app = Router::new()
        .merge(logs_routes)
        .merge(subs_routes)
        .layer(Extension(app_state.clone()))
        .layer(cors);

    Ok(app.into())
}
