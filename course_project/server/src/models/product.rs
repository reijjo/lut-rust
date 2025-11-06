use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
	id: i32,
	title: String,
	price: f64,
	description: String,
	category: String,
	image: String
}
