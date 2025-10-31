#[macro_use] extern crate rocket;

#[post("/receive", data = "<number>")]
fn double_number(number: String) -> String {
    match number.trim() {
       "I eat yellow snow!" => {
				format!("Don't do that!")
			 },
			 _ => format!("Response received: {}", number)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![double_number])
}