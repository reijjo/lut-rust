use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
      .with(
        tracing_subscriber::fmt::layer()
          .with_level(true)
          .with_target(true)
          .with_ansi(true)
          .compact()
      )
      .init();

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