use std::fs::*;
use std::env;
use std::io::{self, Write};

fn main() {
  let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("No arguments were given.");
		return;
	}

	if args.len() < 3 {
		println!("Usage: cargo run <filename> <action>");
		return;
	}

	let file_name = &args[1];
	let action_arg = &args[2];

	if *action_arg == "read" {
		if args.len() != 3 {
			println!("Only give three arguments when writing.");
			return;
		}
		println!("The contents of the file:\n");
		read_file(file_name);
	} else if *action_arg == "write" {

		if args.len() < 4 {
			println!("More arguments needed if not reading.");
			return;
		}

		let content = &args[3];
		let _ = write_in_file(file_name, content);
	} else {
		println!("Last argument must be read/write");
	}
}

fn write_in_file(file_name: &String, input: &String) -> io::Result<()> {
  let mut file = File::create(file_name)?;
  write!(file, "{}", input)?;
  Ok(())
}

fn read_file(file: &String) {
	 match read_to_string(file) {
    Ok(contents) => println!("{}", contents),
    Err(_) => println!("Error opening file.")
  }
}