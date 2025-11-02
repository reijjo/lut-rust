use axum::{
    middleware::{self},
    Router,
};

use crate::routes;
use crate::middleware::logger::log_middleware;

pub fn create_app() -> Router {
  Router::new()
    .merge(routes::init_routes())
    .layer(middleware::from_fn(log_middleware))
}
