#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::response::content::RawHtml;

#[derive(FromForm)]
struct UserInput {
    good: Option<String>,
    bad: Option<String>,
}

#[post("/answer", data = "<user_form>")]
fn handle_form(user_form: Form<UserInput>) -> RawHtml<String> {
    let user = user_form.into_inner();
    let message = match (user.good.is_some(), user.bad.is_some()) {
        (true, true) => "Can you really be having both a good and a bad day at the same time?",
        (true, false) => "Hey, I am glad to hear that. Keep on rocking'! :)",
        (false, true) => "I'm sorry to hear that. I hope things get better for you. :(",
        (false, false) => "You did not share your feelings. :(",
    };

    RawHtml(format!(
        r#"<div id="response"><h2>{}</h2></div>"#,
        message
    ))
}

#[get("/")]
fn form_page() -> RawHtml<&'static str> {
    RawHtml(r#"
      <div>
        <form action="/answer" method="post">
          <h1>How are you?</h1>
          <label for="good">Good</label>
          <input type="checkbox" id="good" name="good" value="good">
          <label for="bad">Bad</label>
          <input type="checkbox" id="bad" name="bad" value="bad">
          <input type="submit" value="Submit answer">
        </form>
      </div>
"#)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![form_page, handle_form])
}