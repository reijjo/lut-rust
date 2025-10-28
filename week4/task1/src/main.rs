use std::fs::*;
use std::io;

fn main() {

	loop {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Error reading input");

		print!("$ ");

		match input.trim() {
			"read" => read_file(),
			"prank" => prank_user(),
			"end" => break,
			"help" => println!("Commands: read, prank, help, end."),
			_ => {
				println!("Invalid command. Try again.");
				continue
			}
		}
	}
}

fn read_file() {
	let file = read_to_string("read.txt").expect("Error opening file.");
	println!("{}\n", file);
}

fn prank_user() {
	println!("You have received an email.");
}
