use std::env;
use std::sync::mpsc;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
  let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("No arguments were given.");
		return;
	}

	if args.len() < 3 {
		println!("Usage: cargo run <character> <seconds>");
		return;
	}

	if args.len() > 3 {
		println!("Too many arguments were given.");
		return;
	}

	println!("Do you want to start or exit?");
	print!("$ ");
	io::stdout().flush().unwrap();

	let mut user_input = String::new();
	io::stdin().read_line(&mut user_input).expect("Failed to read input.");

	match user_input.trim() {
		"start" => the_game(&args),
		"exit" => {
			println!("Goodbye then.");
			std::process::exit(0)
		},
		_ => {
			println!("Invalid input.");
			return;
		}
	}


}

fn the_game(args: &Vec<String>) {
	let (tx, rx) = mpsc::channel();

	let character = &args[1];
	let seconds: i32 = match args[2].parse() {
		Ok(num) => num,
		Err(_) => {
			println!("You call this: '{}' a number?", args[2]);
			return;
		}
	};

	let mut count: i32 = 0;

	let timer = tx.clone();
  thread::spawn(move || {
    for _ in 0..seconds {
      thread::sleep(Duration::from_secs(1));
      timer.send("TICK".to_string()).unwrap();
    }
    timer.send("TIMEUP".to_string()).unwrap();
  });

	let input = tx.clone();
	thread::spawn(move || {
		loop {
    	io::stdout().flush().unwrap();

			let mut user_input = String::new();
			io::stdin().read_line(&mut user_input).expect("Failed to read input.");
			input.send(user_input).unwrap();
		}
	});


	for action in rx {
    let trimmed = action.trim();
    if trimmed == character {
        count += 1;
        println!("Presses: {}", count);

    }

		if trimmed == "TIMEUP" {
        println!("You have managed to press '{}' {} times.", character, count);
        std::process::exit(0);
    }
	}
}
