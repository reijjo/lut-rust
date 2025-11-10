use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
	pub id: u32,
	pub title: String,
	pub price: f64,
	pub description: String,
	pub category: String,
	pub image: String
}
