#[macro_use] extern crate rocket;
use std::collections::HashMap;

use harsh::Harsh;
use rocket::form::Form;
use rocket::State;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

const DB_PATH: &str = "data/db";

struct Globals {
    harsh: Harsh,
}

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

    let db = match sled::open(DB_PATH) {
        Ok(db) => db,
        Err(e) => {
            let e = e.to_string();
            context.insert("error", &e);

            return Template::render("index", &context);
        },
    };

    let id = match db.generate_id() {
        Ok(id) => id,
        Err(e) => {
            let e = e.to_string();
            context.insert("error", &e);

            return Template::render("index", &context);
        },
    };

    match db.insert(id.to_be_bytes(), data.url.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            let e = e.to_string();
            context.insert("error", &e);

            return Template::render("index", &context);
        },
    };

    let url = format!("http://localhost:8000/{}", globals.harsh.encode(&vec![id]));
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

    let id = id.to_be_bytes();

    let db = match sled::open(DB_PATH) {
        Ok(db) => db,
        Err(e) => {
            println!("Error: {}", e);
            return Redirect::to("/");
        },
    };

    let value = match db.get(id) {
        Ok(Some(value)) => value,
        Ok(None) => {
            println!("No value found for {}", hash);
            return Redirect::to("/");
        },
        Err(e) => {
            println!("Error: {}", e);
            return Redirect::to("/");
        },
    };

    let url = match std::str::from_utf8(&value) {
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
        .manage(Globals {
            harsh: Harsh::default(),
        })
        .mount("/", routes![get, post, nav])
        .attach(Template::fairing())
}
