use std::collections::HashMap;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[derive(FromForm)]
pub struct Message {
    message: String,
}

#[post("/submit", data = "<message>")]
pub fn submit(mut cookies: Cookies<'_>, message: Form<Message>) -> Redirect {
    cookies.add(Cookie::new("message", message.into_inner().message));
    Redirect::to("/api/cookie")
}

#[get("/")]
pub fn index(cookies: Cookies<'_>) -> Template {
    let cookie = cookies.get("message");
    let mut context = HashMap::new();
    if let Some(ref cookie) = cookie {
        context.insert("message", cookie.value());
    }

    Template::render("index", &context)
}
