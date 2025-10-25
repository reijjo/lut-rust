use std::io::{self, Write};
use std::fs::File;
use std::collections::HashMap;
use crate::receipt::product::*;

const PRODUCT_1_NAME: &str = "Zbox 720";
const PRODUCT_2_NAME: &str = "GPU - AND Random RT6600";
const PRODUCT_3_NAME: &str = "Potato";
const PRODUCT_1_PRICE: i32 = 600;
const PRODUCT_2_PRICE: i32 = 200;
const PRODUCT_3_PRICE: i32 = 1;

pub struct ReceiptContent {
	pub products: Vec<StoreProduct>,
	pub store: String
}

pub fn store_loop() -> Result<(), ()> {
	let mut cart: Vec<StoreProduct> = Vec::new();
	let mut input = String::new();

	loop {
		println!("| 1) Add to cart | 2) Remove most recent product | 3) Purchase |");

		let parsed_input = read_input(&mut input);

		match parsed_input {
			1 => {
				println!("Which product would you like to add?");
				product_list();

				let product_choice = read_input(&mut input);

				add_to_cart(product_choice, &mut cart);
			},
			2 => {
				cart.pop();
			},
			3 => {
			    let mut receipt = ReceiptContent {
			        products: cart,
			        store: String::from("Repen Valinta Oy")
			    };
			    complete_purchase(&mut receipt)?;
			    println!("Thank you for your purchase!");
			    break Ok(());
			},
			_ => {
				println!("Invalid input. Try again.");
				continue;
			}
		}


	}
}

pub fn product_list() -> Vec<StoreProduct> {
  let products = create_products();
	let mut i: i32 = 1;

	for product in &products {
		println!("{}) {} | Price - {}", i, product.name, product.price);
		i += 1;
	}

	products
}

fn read_input(input: &mut String) -> u8 {
	loop {
		input.clear();

		io::stdin().read_line(input).expect("Error reading input");

		match input.trim().parse() {
			Ok(num) => return num,
			Err(_) => {
				println!("Invalid input. Try again.");
				continue;
			}
		};
	}
}

fn add_to_cart(input: u8, cart: &mut Vec<StoreProduct>) {
	match input {
		1 => {
			cart.push(StoreProduct {
				name: PRODUCT_1_NAME.to_string(),
				price: PRODUCT_1_PRICE
			});
		},
		2 => {
			cart.push(StoreProduct {
				name: PRODUCT_2_NAME.to_string(),
				price: PRODUCT_2_PRICE
			});
		},
		3 => {
			cart.push(StoreProduct {
				name: PRODUCT_3_NAME.to_string(),
				price: PRODUCT_3_PRICE
			});
		},
		_ => println!("Invalid input. Try again.")
	}
}

fn print_receipt(receipt: &ReceiptContent) -> io::Result<()> {
	let mut counts: HashMap<String, (i32, i32)> = HashMap::new();
	let mut total: i32 = 0;

	for product in &receipt.products {
		if let Some((qty, total)) = counts.get_mut(&product.name) {
			*qty += 1;
			*total += product.price;
		} else {
			counts.insert(product.name.clone(), (1, product.price));
		}
	}

	let mut file = File::create("receipt.txt").expect("Unable to create file");

	writeln!(file, "{}", receipt.store).unwrap();
	writeln!(file, "------------------------------").unwrap();

	for (name, (qty, sum)) in &counts {
		writeln!(file, "{} ({}) - {}€", name, qty, sum).unwrap();
		total += sum;
	}

	writeln!(file, "------------------------------").unwrap();
	writeln!(file, "Final price: {}€", total).unwrap();
	writeln!(file, "------------------------------").unwrap();

	Ok(())
}

pub fn complete_purchase(receipt: &mut ReceiptContent) -> Result<(), ()> {
    let _ = print_receipt(&receipt);
    Ok(())
}