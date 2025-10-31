#[macro_use] extern crate rocket;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use std::fs::{self, File};
use std::io::Write;

#[derive(FromForm)]
struct Post {
    posti: String
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    let rendered_page = r#"
    <form action="/message" method="post">
        <label for="posti">Post something</label>
        <input type="text" id="posti" name="posti" required>
        <input type="submit" value="Submit answer">
    </form>
"#;
    RawHtml(rendered_page)
}

#[get("/message")]
fn message() -> RawHtml<String> {
    let file_content = match fs::read_to_string("data.txt") {
        Ok(content) => content,
        Err(_) => {
            let _ = File::create("data.txt");
            String::new()
        }
    };

    let rendered_page = format!(r#"<p>{}</p>"#, file_content);

    RawHtml(rendered_page)
}

#[post("/message", data = "<posti>")]
fn handle_form(posti: Form<Post>) -> String {
    let msg = posti.into_inner();

    let mut file = File::create("data.txt").unwrap();
    write!(file, "{}", msg.posti).unwrap();

    "Message received.".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, message, handle_form])
}