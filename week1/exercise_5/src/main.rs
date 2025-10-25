use rand::Rng;
use std::io;

fn main() {
  let mut health: f32 = 100.0;
	let mut boss_health: f32 = 150.0;
	let mut potions: u8 = 3;

	loop {
		let damage = receive_player_attack_dmg();
		let boss_damage = receive_boss_attack_dmg();
		let mut action = String::new();

		println!("| Your HP - {} | Boss HP - {} |", health, boss_health);
		println!("| 1) Attack | 2) Defend | 3) Heal |");

    io::stdin().read_line(&mut action).expect("Error reading input");

		let parsed_input: u8 = match action.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Parsing failed.");
				continue;
			}
		};

		match parsed_input {
			1 => {
				health -= boss_damage;
				boss_health -= damage;

				println!("Your attack deals {} damage.", damage);
				println!("You take {} damage.", boss_damage);
			},
			2 => {
				let defense = boss_damage * receive_defense_multiplier();
				health -= defense;

				println!("Defense activated!");
				println!("You take {} damage.", defense);
			},
			3 => {
				if potions > 0 {
					potions -= 1;

					health = use_potion(health);
					health -= boss_damage;

					println!("You consume a potion.");
					println!("You take {} damage.", boss_damage);
				} else {
					println!("No potions left.");
					continue;
				}
			}
			_ => {
				println!("Choose a number between 1 - 3.");
				continue;
			}
		}

		if health <= 0.0 {
			println!("You have been defeated!");
			break;
		}

		if boss_health <= 0.0 {
			println!("You win!");
			break;
		}
	}
}

fn receive_player_attack_dmg() -> f32 {
	let mut rng = rand::rng();
	let dmg: f32 = rng.random_range(12.5..=20.0);

	return dmg
}

fn receive_defense_multiplier() -> f32 {
	let mut rng= rand::rng();
	let def = rng.random_range(2.0..=4.0);

	return 1.0 / def
}

fn receive_boss_attack_dmg() -> f32 {
	let mut rng = rand::rng();
	let dmg: f32 = rng.random_range(5.0..=25.0);

	return dmg
}

fn use_potion(health: f32) -> f32 {
	return health + 25.0
}
