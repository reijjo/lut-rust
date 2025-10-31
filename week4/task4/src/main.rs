use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
  let million: i64 = 1_000_000;
	let satak: i64 = 100_000;
	let mut money: i64 = 0;

	println!("Do you have a million dollars? | y = yes, n = no");

	io::stdout().flush().unwrap();

	let mut user_input = String::new();

	io::stdin().read_line(&mut user_input).expect("Failed to read input.");

	match user_input.trim() {
		"y" => {
			println!("All right then, millionaire");
			money += million;
			create_threads(money);
		},
		"n" => {
			println!("Let's just assume you have $100,000 then.");
			money += satak;
			create_threads(money);
		},
		_ => {
			println!("Invalid input.");
			return;
		}
	}
}

fn create_threads(money: i64) {
	let (tx, rx) = mpsc::channel();

	let money_at_start = money;
	let mut money_left = money;

	let big_thief = tx.clone();

	thread::spawn(move || {
		loop {
			thread::sleep(Duration::from_secs(3));

			big_thief.send(35_000).unwrap();
		}
	});

	thread::sleep(Duration::from_millis(100));

	let small_thief = tx.clone();
	thread::spawn(move || {
		loop {
			thread::sleep(Duration::from_secs(5));

			small_thief.send(10_000).unwrap();
		}
	});

	thread::spawn(move || {
		loop {
			io::stdout().flush().unwrap();

			let mut user_input = String::new();

			io::stdin().read_line(&mut user_input).expect("Failed to read input.");

			match user_input.trim() {
				"catch" => {
					println!("The thieves have left.");
					std::process::exit(0);
				},
				_ => {
					continue;
				}
			}
		}
	});

	drop(tx);

	for received_msg in rx {
		if money_at_start == 1_000_000 && money_left <= 600_000 && received_msg == 35_000 {
      continue;
    }

		if received_msg > 10_000 {
			println!("ALERT!!! Someone stole $35,000 from you!");
		} else {
			println!("ALERT!!! Someone stole $10,000 from you!");
		}

		money_left -= received_msg;

		if money_left <= 0 {
			println!("You lost all your money!");
			std::process::exit(0);
		}

		println!("Funds left: {}", money_left);

		if money_at_start == 1_000_000 && money_left <= 550_000 {
			println!("The thieves have left.");
			std::process::exit(0);
		}

	}
}