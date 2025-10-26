mod game;
use game::Country::*;
use game::Player::*;
use std::io;

fn main() {
  let finland = Country::new(String::from("Finland"), 5600000, 900000, vec![], false);
  let sweden = Country::new(String::from("Sweden"), 10000000, 200000, vec![], false);
  let norway = Country::new(String::from("Norway"), 5500000, 100000, vec![], false);
  let denmark = Country::new(String::from("Denmark"), 6000000, 50000, vec![], false);

	let mut input = String::new();

	println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
	println!("Choose your country: ");

	let country_selection = read_input_number(&mut input);

	let selected_country = match country_selection {
		1 => finland,
		2 => sweden,
		3 => norway,
		4 => denmark,
		_ => {
			println!("Invalid input. Please try again.");
			return;
		}
	};

	let player = Player::new(selected_country);

	println!("| Inspection on your own nation? | y = yes | n = no |");
	let selection = read_input_string(&mut input);

	match selection.as_str() {
		"y" => player.inspect(),
		"n" => println!("The leader is confident. No inspection needed."),
		_ => {
			println!("Invalid input. Try again");
			return;
		}
	}

}

fn print_country(country: &Country) {
	println!("Country: {}", country.get_name());
	println!("Population: {}", country.get_population());
	println!("Army size: {}", country.get_army_size());
}

fn read_input_number(input: &mut String) -> u8 {
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

fn read_input_string(input: &mut String) -> String {
	input.clear();

	io::stdin().read_line(input).expect("Error reading input");
	input.trim().to_string()
}
