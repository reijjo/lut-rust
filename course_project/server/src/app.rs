use axum::{
  middleware::{self},
  Router,
};
use std::sync::{Arc, Mutex};

use crate::routes;
use crate::middleware::logger::log_middleware;
use crate::models::cart::Cart;

// Mutex = Only one can read/write at a time
// Arc = Shareable acress entire server
#[derive(Clone)]
pub struct AppState {
	pub http_client: reqwest::Client,
	pub cart: Arc<Mutex<Cart>>
}

pub fn create_app() -> Router {
	let http_client = reqwest::Client::builder()
		.timeout(std::time::Duration::from_secs(10))
		.build()
		.expect("Failed to create HTTP client");

	let cart = Arc::new(Mutex::new(Cart::new()));

	let state = AppState {
		http_client,
		cart
	};


  Router::new()
    .merge(routes::init_routes())
    .layer(middleware::from_fn(log_middleware))
		.with_state(state)
}
