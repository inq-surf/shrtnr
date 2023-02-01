#[macro_use] extern crate rocket;
use std::collections::HashMap;

use rocket::form::Form;
use rocket::State;
use rocket::http::uri::{Uri, Absolute};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

mod global;
mod shrtnr;

use global::Globals;

#[derive(FromForm)]
struct Url {
    url: String,
}

#[get("/")]
fn get() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();

    Template::render("index", &context)
}

#[post("/", data = "<data>")]
fn post(globals: &State<Globals>, data: Form<Url>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();

    match Uri::parse::<Absolute>(&data.url) {
        Ok(_) => (),
        Err(e) => {
            let e = e.to_string();
            context.insert("error", &e);

            return Template::render("index", &context);
        },
    };

    let id = match shrtnr::add(&data.url) {
        Ok(id) => id,
        Err(e) => {
            context.insert("error", &e);

            return Template::render("index", &context);
        },
    };

    let url = format!(
        "{}/{}",
        globals.shrtnr_config.host,
        globals.harsh.encode(&vec![id])
    );
    context.insert("url", &url);

    Template::render("index", &context)
}

#[get("/<hash>")]
fn nav(globals: &State<Globals>, hash: &str) -> Redirect {
    let id = match globals.harsh.decode(hash) {
        Ok(id) => id,
        Err(e) => {
            println!("Error: {}", e);
            return Redirect::to("/");
        },
    };

    let id = match id.first() {
        Some(id) => id,
        None => {
            println!("Error: No id found");
            return Redirect::to("/");
        },
    };

    let url = match shrtnr::get(id) {
        Ok(url) => url,
        Err(e) => {
            println!("Error: {}", e);
            return Redirect::to("/");
        },
    };

    Redirect::to(url.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Globals::new())
        .mount("/", routes![get, post, nav])
        .attach(Template::fairing())
}
