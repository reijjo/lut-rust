pub mod health;

use axum::Router;

pub fn init_routes() -> Router {
  Router::new().merge(health::routes())
}
