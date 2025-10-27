mod game;
use game::Country::*;
use game::Player::*;
use game::GameMap::*;
use std::io;

fn main() {
	let game_map = GameMap::new();
	let mut input = String::new();

	println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
	println!("Choose your country: ");

	let country_selection = read_input_number(&mut input);

	let selected_country = match country_selection {
		1 => game_map.get_country_by_index(1).clone(),
		2 => game_map.get_country_by_index(3).clone(),
		3 => game_map.get_country_by_index(2).clone(),
		4 => game_map.get_country_by_index(0).clone(),
		_ => {
			println!("Invalid input. Please try again.");
			return;
		}
	};

	let player = Player::new(selected_country);

	loop {
		println!("| Inspection on your own nation? | y = yes | n = no |");
		let selection = read_input_string(&mut input);

		match selection.as_str() {
			"y" => player.inspect(),
			"n" => println!("The leader is confident. No inspection needed."),
			_ => {
				println!("Invalid input. Try again");
				continue;
			}
		}

		println!("| 1) Spy on a country | 0) Exit program |");
    let spy_selection = read_input_number(&mut input);

    match spy_selection {
      1 => {
        game_map.list_countries();
        let country_to_spy = read_input_number(&mut input);

        let spy_target = match country_to_spy {
          1 => game_map.get_country_by_index(0),
          2 => game_map.get_country_by_index(1),
          3 => game_map.get_country_by_index(2),
          4 => game_map.get_country_by_index(3),
          _ => {
            println!("Invalid input. Try again.");
            continue;
          }
        };

        player.spy(spy_target);
      },
      0 => break,
      _ => println!("Invalid game input. Try again.")
    }
	}
}

// fn select_country(input: &mut String) -> Country {

// }

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
