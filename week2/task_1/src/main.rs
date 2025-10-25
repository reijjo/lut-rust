use std::io;

const TEXT: &str = "I want to be changed.";

fn main() {
	let mut text = create_default();

	loop {
		let mut choise = String::new();

		println!("| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |");

		io::stdin().read_line(&mut choise).expect("Error reading input");

		let parsed_choise: u8 = match choise.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Parsing error. Was the input a number?");
				println!("| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |");
				continue;
			}
		};

		match parsed_choise {
			1 => text = create_default(),
			2 => remove_latest_word(&mut text),
			3 => {
				let mut new_word = String::new();
				println!("The new word: ");

				io::stdin().read_line(&mut new_word).expect("Error reading input");

				let new_word = new_word.trim();

				text.push(' ');
				text.push_str(&new_word);

			},
			4 => println!("{}", text),
			0 => break,
			_ => println!("Invalid input. Try again.")
		}
	}
}

fn create_default() -> String {
	String::from(TEXT)
}

fn remove_latest_word(input: &mut String) {
    if let Some(pos) = input.rfind(' ') {
        input.truncate(pos);
    } else {
        input.clear();
    }
}
