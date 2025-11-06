use axum::{
	extract::{State},
  http::StatusCode,
	routing::get,
	Json, Router
};
use serde_json::json;

use crate::models::cart::Cart;
use crate::routes::AppState;

pub fn cart_routes() -> Router<AppState> {
	Router::new()
		.route("/", get(get_cart))
}

// GET cart
async fn get_cart(
	State(state): State<AppState>
) -> Result<Json<Cart>, (StatusCode, Json<serde_json::Value>)> {
	// If there is a Mutex -> Use .lock to access it
	let cart = match state.cart.lock() {
		Ok(cart) => cart,
		Err(e) => {
			tracing::error!("Failed to fetch cart: {}", e);

			return Err((
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(json!({ "error": "Failed to access cart" }))
			));
		}
	};	// <-- The lock goes out of scope and others can now read/write it

	Ok(Json(cart.clone()))	// .clone() because we cant move the data out of the mutex
}