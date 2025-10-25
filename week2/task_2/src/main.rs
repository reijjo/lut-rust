use std::io;

fn main() {
	let mut number: i32 = 0;

  loop {
		let mut choise = String::new();

		println!("| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |");

		io::stdin().read_line(&mut choise).expect("Error reading input");

		let parsed_choise: u8 = match choise.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Invalid input. Try again.");
				continue;
			}
		};

		match parsed_choise {
			1 => number = 0,
			2 => {
				let value = get_input();
				add(&mut number, value);
			},
			3 => {
				let value = get_input();
				sub(&mut number, value);
			},
			4 => {
				let value = get_input();
				multi(&mut number, value);
			},
			5 => {
				let value = get_input();
				division(&mut number, value);
			},
			6 => println!("Current number: {}", number),
			0 => {
				println!("Ending the program.");
				break;
			},
			_ => {
				println!("Invalid input. Try again.");
				continue;
			}
		}
	}
}

fn add(number: &mut i32, input: i32) {
	*number += input
}

fn sub(number: &mut i32, input: i32) {
	*number -= input
}

fn multi(number: &mut i32, input: i32) {
	*number *= input
}

fn division(number: &mut i32, input: i32) {
	if input == 0 {
		println!("Cannot divide by 0.");
		return;
	}

	*number /= input
}

fn get_input() -> i32 {
	println!("What number?");

	loop {
		let mut input = String::new();

		io::stdin().read_line(&mut input).expect("Error reading input");

		match input.trim().parse() {
			Ok(num) => return num,
			Err(_) =>	println!("Invalid input. Try again.")
		};
	}
}