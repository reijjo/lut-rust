use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
	id: i32,
	title: String,
	pub price: f64,
	description: String,
	category: String,
	image: String
}
