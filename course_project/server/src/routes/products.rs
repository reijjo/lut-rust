use axum::{
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde_json::json;
use crate::models::product::Product;

pub fn product_routes() -> Router {
  Router::new()
    .route("/", get(get_products))
}

async fn get_products() -> Result<Json<Vec<Product>>, (StatusCode, 			Json<serde_json::Value>)> {
  let client = reqwest::Client::new();

	let response = match client.get("https://fakestoreapi.com/products").send().await {
  	Ok(resp) => resp,
  	Err(e) => {
      tracing::error!("Failed to fetch products: {}", e);
      return Err((
        StatusCode::BAD_GATEWAY,
        Json(json!({ "error": "Failed to connect to products API" }))
      ));
  	}
	};

  let products = match response.json::<Vec<Product>>().await {
    Ok(data) => data,
    Err(e) => {
      tracing::error!("Failed to parse products: {}", e);
      return Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "Failed to parse products data" }))
      ));
    }
  };

  Ok(Json(products))
}