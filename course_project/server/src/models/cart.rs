use serde::{Deserialize, Serialize};

use crate::models::product::Product;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartProduct {
	#[serde(flatten)]	// makes it look like extended type in TypeScript
	pub product: Product,
	pub quantity: u32
}

impl CartProduct {
	pub fn set_quantity(&mut self, quantity: u32) {
		self.quantity = quantity
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cart {
	pub products: Vec<CartProduct>,
	pub total: f64
}

impl Cart {
	pub fn new() -> Self {
		Self {
			products: Vec::new(),
			total: 0.0
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct UpdateQuantity {
	pub quantity: u32
}