#[macro_use] extern crate rocket;
use std::collections::HashMap;

use harsh::Harsh;
use rocket::form::Form;
use rocket::State;
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
fn post(globals: &State<Globals>, data: Form<Url>) -> String {
    let db = match sled::open(DB_PATH) {
        Ok(db) => db,
        Err(e) => {
            return format!("Error: {}", e);
        },
    };

    let id = match db.generate_id() {
        Ok(id) => id,
        Err(e) => {
            return format!("Error: {}", e);
        },
    };

    match db.insert(id.to_be_bytes(), data.url.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            return format!("Error: {}", e);
        },
    };

    format!("http://localhost:8000/{}", globals.harsh.encode(&vec![id]))
}

#[get("/<hash>")]
fn nav(globals: &State<Globals>, hash: &str) -> String {
    let id = match globals.harsh.decode(hash) {
        Ok(id) => id,
        Err(e) => {
            return format!("Error: {}", e);
        },
    };

    let id = match id.first() {
        Some(id) => id,
        None => {
            return format!("Error: No id found");
        },
    };

    let id = id.to_be_bytes();

    let db = match sled::open(DB_PATH) {
        Ok(db) => db,
        Err(e) => {
            return format!("Error: {}", e);
        },
    };

    let value = match db.get(id) {
        Ok(Some(value)) => value,
        Ok(None) => {
            return format!("No value found for {}", hash);
        },
        Err(e) => {
            return format!("Error: {}", e);
        },
    };

    let url = match std::str::from_utf8(&value) {
        Ok(url) => url,
        Err(e) => {
            return format!("Error: {}", e);
        },
    };

    url.to_string()
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
