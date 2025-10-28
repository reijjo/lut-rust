use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
  thread::spawn(|| {
		start_duel();
	});

	io::stdout().flush().unwrap();

	let mut user_input = String::new();

	io::stdin().read_line(&mut user_input).expect("Failed to read input");

	match user_input.trim() {
		"f" => println!("You fire first!"),
		_ => {
			thread::sleep(Duration::from_secs(5));
		}
	}
}

fn start_duel() {
	thread::sleep(Duration::from_secs(5));
	println!("FIRE!!!");
}