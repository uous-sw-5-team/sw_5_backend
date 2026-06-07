mod db;
mod error;
mod models;
mod routes;
mod auth;

use std::sync::Arc;
use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use db::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "planner_backend=debug,info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Database::connect().await?;
    let state = AppState { db: Arc::new(db) };

    // uploads 디렉토리 생성
    tokio::fs::create_dir_all("uploads").await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", routes::all_routes(state))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("서버 시작: http://0.0.0.0:8080");
    axum::serve(listener, app).await?;

    Ok(())
}
