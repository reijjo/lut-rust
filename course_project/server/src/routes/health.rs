use axum::{routing::get, Json, Router};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::time::Instant;

static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    uptime: f64,
}

pub fn routes() -> Router {
    let _ = *START_TIME; // initialize timer
    Router::new().route("/up", get(health_check))
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "UP".to_string(),
        uptime: START_TIME.elapsed().as_secs_f64(),
    })
}
