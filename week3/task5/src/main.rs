mod game;
use game::Country::*;
use game::Player::*;
use game::GameMap::*;
use std::io;

fn main() {
	let mut game_map = GameMap::new();
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

	let mut player = Player::new(selected_country);

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

		println!("| 1) Spy on a country | 2) Invade a country | 3) Expand military |");
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
			2 => {
				game_map.list_countries();
				let country_to_conquer = read_input_number(&mut input);

        let target = match country_to_conquer {
					1 => game_map.get_country_by_index(0),
          2 => game_map.get_country_by_index(1),
          3 => game_map.get_country_by_index(2),
          4 => game_map.get_country_by_index(3),
          _ => {
						println!("Invalid input. Try again.");
            continue;
          }
				};

				let my_country = player.get_country().get_name().to_string();
				player.conquer_nation(target, &my_country);

				if player.get_country().get_is_conquered() {
					println!("Game over!");
					break;
				}
			},
			3 => {
				let country = player.get_country();
				country.extra_army()
			},
      0 => break,
      _ => println!("Invalid game input. Try again.")
    }

		let country_name = player.get_country().get_name().to_string();
		game_map.other_countries_turn(&country_name);

		let conquered_nations = player.get_country().get_conquered_nations();
		if conquered_nations.len() == 3 {
			println!("You have conquered all your targets. Good work, comrade!");
			break;
		}
	}
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
