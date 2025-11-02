use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

mod app;
mod utils;
mod routes;
mod models;
mod middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    utils::tracing::init_tracing();

    let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("{}:{}", host, port);

    tracing::info!("Environment: '{}'", environment);
    tracing::info!("🚀 Server starting on http://{}", addr);

    let app = app::create_app();
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}