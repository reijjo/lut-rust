use serde::{Deserialize, Serialize};

use crate::models::product::Product;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartProduct {
	#[serde(flatten)]	// makes it look like extended type in TypeScript
	product: Product,
	quantity: i32
}

impl CartProduct {
	pub fn new(product: Product, quantity: i32) -> Self {
		Self { product, quantity }
	}

	pub fn set_quantity(&mut self, quantity: i32) {
		self.quantity = quantity
	}

	pub fn subtotal(&self) -> f64 {
		self.product.price * self.quantity as f64
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

	pub fn set_total(&mut self, total: f64) {
		self.total = total
	}
}