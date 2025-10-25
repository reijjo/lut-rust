mod game;
use game::Country::*;
use std::io;

fn main() {
  let finland = Country::new(String::from("Finland"), 5600000, 900000, vec![], false);
  let sweden = Country::new(String::from("Sweden"), 10000000, 200000, vec![], false);
  let norway = Country::new(String::from("Norway"), 5500000, 100000, vec![], false);
  let denmark = Country::new(String::from("Denmark"), 6000000, 50000, vec![], false);

	let mut input = String::new();

	loop {
		println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
		println!("Choose your country: ");

		let country_selection = read_input(&mut input);

		match country_selection {
			1 => print_country(&finland),
			2 => print_country(&sweden),
			3 => print_country(&norway),
			4 => print_country(&denmark),
			_ => {
				println!("Invalid input. Please try again.");
			}
		}
		break;
	}
}

fn print_country(country: &Country) {
	println!("Country: {}", country.get_name());
	println!("Population: {}", country.get_population());
	println!("Army size: {}", country.get_army_size());
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
