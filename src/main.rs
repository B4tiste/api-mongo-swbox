use axum::{Extension, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;

use std::{env, net::SocketAddr};

use api_mongo_swbox::{
    routes::logs,
    state::AppState,
};

fn env_required(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing required env var: {key}"))
}

#[tokio::main]
async fn main() {
    // Charge .env si présent (local / raspberry). Même pattern que ton bot. :contentReference[oaicite:0]{index=0}
    let _ = dotenvy::dotenv();

    // Mongo
    let mongo_uri = env_required("MONGO_URI");
    let mongo = mongodb::Client::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");

    let app_state = AppState { mongo };

    // CORS (tu peux aussi le rendre configurable via ENV plus tard)
    let cors = CorsLayer::new()
        .allow_origin([
            "https://bot-swbox.netlify.app".parse().unwrap(),
            "http://localhost:4200".parse().unwrap(),
            "http://127.0.0.1:4200".parse().unwrap(),
            "http://127.0.0.1:8000".parse().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    // Routers
    let logs_routes = logs::logs_routes(app_state.clone());

    let app = Router::new()
        .merge(logs_routes)
        .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1MB
        .layer(Extension(app_state))
        .layer(cors);

    // Serve
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}
