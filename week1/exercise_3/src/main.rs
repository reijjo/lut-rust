use std::io;

fn main() {
    let mut current: i32 = 0;
    let max_i16 = i16::MAX as i32;

    println!("By how much do you want to increment the number?");

    loop {
        println!("Current: {}. Increment by: ", current);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");

        let parsed_input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing failed. Was the number too long for a 16-bit variable?");
                continue;
            }
        };

        if parsed_input == 0 {
            println!("The given value is 0. Ending the program.");
            break;
        }

        if parsed_input < 0 {
            println!("The given value is lower than 0.");
            continue;
        }

        current += parsed_input;

        if current > max_i16 {
            println!("Enough incrementations.");
            break;
        }
    }
}
