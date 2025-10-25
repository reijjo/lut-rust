use rand::Rng;

fn main() {
  let mut counter = 1;

	loop {
		let what = receive_random();

		if what <= 3 {
			println!("Low...");
		} else if what >= 4 && what <= 6 {
			println!("Middle!");
		} else if what >= 7 && what <= 9 {
			println!("High!!");
		} else if what == 10 {}

		if what == 10 {
			println!("Jackpot!!!");
			println!("{}", measure_luck(counter));
			break;
		}
		counter += 1;
	}
}

fn receive_random() -> u8 {
	let mut rng = rand::rng();
	return rng.random_range(1..=10)
}

fn measure_luck(value: i32) -> String {
	if value > 3 {
		return String::from("You were UNLUCKY!")
	} else {
		return String::from("You were LUCKY!")
	}
}