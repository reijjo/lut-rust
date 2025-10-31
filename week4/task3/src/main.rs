use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::sync::mpsc;

fn main() {
	start_duel();
}

fn start_duel() {
	let (tx, rx) = mpsc::channel();

	let duel = tx.clone();
	thread::spawn(move || {
		thread::sleep(Duration::from_secs(5));
		duel.send("FIRE!!!").unwrap();

		io::stdout().flush().unwrap();

		let mut user_input = String::new();

		io::stdin().read_line(&mut user_input).expect("Failed to read input");

		match user_input.trim() {
			"f" => {
				duel.send("You fire first!").unwrap();
				std::process::exit(0);
			},
			_ => {
				duel.send("Oh no! You missed!").unwrap();
				duel.send("Opponent shoots first!").unwrap();
				thread::sleep(Duration::from_secs(5));
			}
		}
	});

	let opponent = tx.clone();
	thread::spawn(move || {
		thread::sleep(Duration::from_secs(8));
		opponent.send("Opponent shoots first!").unwrap();
		std::process::exit(0);
	});

	for received_msg in rx {
		println!("{received_msg}");
	}
}