use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{Response},
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use std::time::Instant;
use once_cell::sync::Lazy;

// Initialize at app start
static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    uptime: f64,
}

pub fn create_app() -> Router {
    // Force START_TIME to initialize now
    let _ = *START_TIME;

    Router::new()
        .route("/up", get(health_check))
        .layer(middleware::from_fn(log_middleware))
}

async fn health_check() -> Json<HealthResponse> {
    let uptime = START_TIME.elapsed().as_secs_f64();

    Json(HealthResponse {
        status: "UP".to_string(),
        uptime,
    })
}

async fn log_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    tracing::info!("→ {} {}", method, path);

    let response = next.run(req).await;
    let status = response.status();

    tracing::info!("← {} {} {}", method, path, status.as_u16());

    Ok(response)
}