use serde::{Deserialize, Serialize};

use crate::models::product::Product;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartProduct {
	#[serde(flatten)]	// makes it look like extended type in TypeScript
	product: Product,
	quantity: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cart {
	pub products: Vec<CartProduct>,
	pub total: f64
}