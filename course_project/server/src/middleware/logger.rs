use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

pub async fn log_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    tracing::info!("→ {} {}", method, path);

    let response = next.run(req).await;
    tracing::info!("← {} {} {}", method, path, response.status().as_u16());

    Ok(response)
}
