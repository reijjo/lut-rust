pub mod health;
pub mod products;
use axum::Router;

pub fn init_routes() -> Router {
  Router::new()
		.merge(health::routes())
		.nest("/api/products", products::product_routes())
}
