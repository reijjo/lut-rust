use axum::{
	extract::{State, Path},
  http::StatusCode,
	routing::{get, post, patch, delete},
	Json, Router
};
use serde_json::json;

use crate::models::cart::{Cart, CartProduct, UpdateQuantity};
use crate::models::product::Product;
use crate::app::AppState;

pub fn cart_routes() -> Router<AppState> {
	Router::new()
		.route("/", get(get_cart))
		.route("/", post(add_to_cart))
		.route("/", delete(clear_cart))
		.route("/{id}", patch(update_cart_item))
		.route("/{id}", delete(delete_cart_item))
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
	State(state): State<AppState>,
	Json(cart_product): Json<CartProduct>
) -> Result<Json<Cart>, (StatusCode, Json<serde_json::Value>)> {
	let CartProduct { product, quantity } = &cart_product;
	let Product { id, title, price, description, category, image } = product;

	if *id == 0 || title.is_empty() || *price <= 0.0 || description.is_empty() ||
		 category.is_empty() || image.is_empty() || *quantity <= 0 {
    return Err((
      StatusCode::BAD_REQUEST,
      Json(serde_json::json!({
        "error": "Invalid product data: all fields must be valid and non-empty"
      }))
    ));
  }

	// Acquire MUTABLE lock on cart (blocks other threads from reading/writing)
	// All operations below are thread-safe while lock is held
	let mut cart = match state.cart.lock() {
		Ok(cart) => cart,
		Err(e) => {
			// Log detailed error for debugging (includes PoisonError details)
			tracing::error!("Cart mutex poisoned while adding item: {}", e);
			return Err((
				StatusCode::INTERNAL_SERVER_ERROR,
				// Generic message to client (don't expose internal details)
				Json(json!({ "error": "Unable to add item to cart" }))
			));
		}
	};

	// Check if product already exists in cart
	if let Some(existing_product) = cart.products
		.iter_mut()
		.find(|p| p.product.id == *id) {
			// Product exists: increment quantity
			existing_product.quantity += quantity;
	} else {
		// Product doesn't exist: add new item to cart
		cart.products.push(cart_product);
	}

	// Recalculate total price for all items
	cart.total = cart.products.iter()
		.map(|p| p.product.price * p.quantity as f64)
		.sum::<f64>()
		.max(0.0);

	Ok(Json(cart.clone()))
}

// Patch cart/:id
async fn update_cart_item(
	State(state): State<AppState>,
	Path(id): Path<u32>,
	Json(item_quantity): Json<UpdateQuantity>
) -> Result<Json<Cart>, (StatusCode, Json<serde_json::Value>)> {
	let quantity = item_quantity.quantity;

	if quantity <= 0 {
		return Err((
			StatusCode::BAD_REQUEST,
			Json(json!({ "error": "Quantity must be greater than 0" }))
		));
	}

	let mut cart = match state.cart.lock() {
		Ok(cart) => cart,
		Err(e) => {
			tracing::error!("Cart mutex poisoned while updating item: {}", e);
			return Err((
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(json!({ "error": "Unable to update item in cart" }))
			));
		}
	};

	if let Some(product_to_update) = cart.products
		.iter_mut()
		.find(|p| p.product.id == id) {
			product_to_update.set_quantity(quantity);

			cart.total = cart.products.iter()
				.map(|p| p.product.price * p.quantity as f64)
				.sum::<f64>()
				.max(0.0);
	} else {
		return Err((
			StatusCode::NOT_FOUND,
			Json(json!({ "error": "Product not found in cart" }))
		));
	}

	Ok(Json(cart.clone()))
}

// Delete cart/:id
async fn delete_cart_item(
	State(state): State<AppState>,
	Path(id): Path<u32>
) -> Result<Json<Cart>, (StatusCode, Json<serde_json::Value>)> {

	let mut cart = match state.cart.lock() {
		Ok(cart) => cart,
		Err(e) => {
			tracing::error!("Cart mutex poisoned while deleting item: {}", e);
			return Err((
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(json!({ "error": "Unable to delete item from cart" }))
			));
		}
	};

	// Check if the product exists before deleting
	let product_exists = cart.products
		.iter()
		.any(|p| p.product.id == id);

	if !product_exists {
		return Err((
			StatusCode::NOT_FOUND,
			Json(json!({ "error": "Product not found in cart" }))
		));
	}

	// Remove the product from cart
	cart.products.retain(|p| p.product.id != id);

	cart.total = cart.products.iter()
		.map(|p| p.product.price * p.quantity as f64)
		.sum::<f64>()
		.max(0.0);

	Ok(Json(cart.clone()))
}

async fn clear_cart(
  State(state): State<AppState>
) -> Result<Json<Cart>, (StatusCode, Json<serde_json::Value>)> {
  let mut cart = match state.cart.lock() {
    Ok(cart) => cart,
    Err(e) => {
      tracing::error!("Cart mutex poisoned while clearing cart: {}", e);
      return Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "Unable to clear cart" }))
      ));
    }
  };

  cart.products.clear();
  cart.total = 0.0;

  Ok(Json(cart.clone()))
}