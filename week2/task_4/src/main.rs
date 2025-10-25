pub mod map;
use map::arraymap::*;
use std::io;

fn main() {
	let mut map = create_map();
	let (mut x, mut y) = (2, 2);

	map[x][y] = 'x';

	update_map(x, y);

	loop {
		let mut input = String::new();

		println!("| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |");

		io::stdin().read_line(&mut input).expect("Error reading input");

		match input.trim().to_lowercase().as_str() {
			"w" => {
				if x > 0 {
					x -= 1;
				} else {
					println!("Can't move out of the map");
				}
				update_map(x, y);
			},
			"a" => {
				if y > 0 {
					y -= 1;
				} else {
					println!("Can't move out of the map");
				}
				update_map(x, y);
			},
			"s" => {
				if x < 4 {
					x += 1;
				} else {
					println!("Can't move out of the map");
				}
				update_map(x, y);
			},
			"d" => {
				if y < 4 {
					y += 1;
				} else {
					println!("Can't move out of the map");
				}
				update_map(x, y);
			},
			"e" => {
				println!("Ending the program.");
				break;
			},
			_ => {
				println!("Invalid input.");
				continue;
			}
		}
	}
}
