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

pub async fn create_app() -> Router {
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

async fn log_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = next.run(req).await;
    let status = response.status();

    let (parts, body) = response.into_parts();
    let bytes = to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Pretty print JSON in logs
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&bytes) {
        let pretty = serde_json::to_string_pretty(&json).unwrap_or_else(|_| String::from_utf8_lossy(&bytes).to_string());
        tracing::info!(
            "\n{} {} {}\nResponse:\n{}",
            method,
            uri.path(),
            status.as_u16(),
            pretty
        );
    } else {
        let body_str = String::from_utf8_lossy(&bytes);
        tracing::info!(
            "{} {} {} - Response: {}",
            method,
            uri.path(),
            status.as_u16(),
            body_str
        );
    }

    let response = Response::from_parts(parts, Body::from(bytes));
    Ok(response)
}