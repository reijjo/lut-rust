use axum::{
		extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::models::product::Product;
use crate::app::AppState;

pub fn product_routes() -> Router<AppState> {
  Router::new()
    .route("/", get(get_products))
		.route("/{id}", get(get_product_by_id))
}

async fn get_products(
	State(state): State<AppState>
) -> Result<Json<Vec<Product>>, (StatusCode, Json<serde_json::Value>)> {
	let response = match state.http_client
		.get("https://fakestoreapi.com/products")
		.send()
		.await
	{
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

async fn get_product_by_id(
	State(state): State<AppState>, Path(id): Path<u32>
) -> Result<Json<Product>, (StatusCode, Json<serde_json::Value>)> {
	let url = format!("https://fakestoreapi.com/products/{}", id);

	let response = match state.http_client
		.get(&url)
		.send()
		.await
	{
		Ok(res) => res,
		Err(e) => {
			tracing::error!("Failed to fetch product with {}: {}", id, e);
			return Err((
				StatusCode::BAD_GATEWAY,
				Json(json!({ "error": "Failed to connect to products API" }))
			));
		}
	};


	if response.status() == reqwest::StatusCode::NOT_FOUND {
		return Err((
			StatusCode::NOT_FOUND,
			Json(json!({ "error": format!("Product with id {} not found", id) }))
		));
	}

	let product = match response.json::<Product>().await {
		Ok(data) => data,
		Err(e) => {
			tracing::error!("Failed to parse product {}: {}", id, e);
			return Err((
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(json!({ "error": "Failed to parse product data" }))
			));
		}
	};

	Ok(Json(product))
}