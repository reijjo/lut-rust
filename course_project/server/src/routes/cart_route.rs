use axum::{
	extract::{State},
  http::StatusCode,
	routing::{get, post},
	Json, Router
};
use serde_json::json;

use crate::models::cart::{Cart, CartProduct};
use crate::models::product::Product;
use crate::app::AppState;

pub fn cart_routes() -> Router<AppState> {
	Router::new()
		.route("/", get(get_cart))
		.route("/", post(add_to_cart))
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

// POST cart
async fn add_to_cart(
	State(state): State<AppState>, Json(cart_product): Json<CartProduct>
) -> Result<Json<Cart>, (StatusCode, Json<serde_json::Value>)> {
	let CartProduct { product, quantity } = &cart_product;
	let Product { id, title, price, description, category, image } = product;

	if *id == 0 || title.is_empty() || *price <= 0.0 || description.is_empty() ||
		 category.is_empty() || image.is_empty() || *quantity <= 0 {
    return Err((
      StatusCode::BAD_REQUEST,
      Json(serde_json::json!({
        "error": "All product fields are required"
      }))
    ));
  }

	// Adding the product to cart is done later
	let cart = match state.cart.lock() {
		Ok(cart) => {
			println!("product title {} x {}", cart_product.product.title, cart_product.quantity);
			cart
		},
		Err(e) => {
			tracing::error!("Failed to add item to cart: {}", e);

			return Err((
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(json!({ "error": "Failed to add item" }))
			));
		}
	};


	Ok(Json(cart.clone()))
}