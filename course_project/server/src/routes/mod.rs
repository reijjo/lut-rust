pub mod health;
pub mod products;
pub mod cart_route;

use axum::Router;
use crate::app::AppState;

pub fn init_routes() -> Router<AppState> {
  Router::new()
		.merge(health::routes())
		.nest("/api/products", products::product_routes())
		.nest("/api/cart", cart_route::cart_routes())
}
