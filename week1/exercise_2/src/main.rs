use std::io;

fn main() {
  let rust = "rust";
	let no = "no";

	println!("Give me a word:");

	let mut sana = String::new();
	io::stdin().read_line(&mut sana).expect("Virhe syötteen lukemisessa");
	let sana = sana.trim().to_lowercase();

	if sana == rust {
		println!("So you appreciate Rust? That's great! Thank you!");
	} else if sana == no {
		println!("So you like nothing? Alright then... :)");
	} else {
		println!("It seems that you like {}.", sana);
	}
}
