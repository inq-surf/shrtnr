#[macro_use] extern crate rocket;
use std::collections::HashMap;

use config::Config;
use harsh::Harsh;
use rocket::form::Form;
use rocket::State;
use rocket::http::uri::{Uri, Absolute};
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

#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
struct HarshConfig {
    salt: Option<String>,
    length: Option<usize>,
    alphabet: Option<String>,
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

fn get_harsh() -> Harsh {
    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("HARSH")
                .try_parsing(true)
                .separator("__")
                .ignore_empty(true)
        )
        .build();

    let config = match config {
        Ok(config) => config,
        Err(e) => {
            println!("Error: {}", e);
            return Harsh::default();
        },
    };

    let config: HarshConfig = match config.try_deserialize() {
        Ok(config) => config,
        Err(e) => {
            println!("Error: {}", e);
            return Harsh::default();
        },
    };

    let mut builder = Harsh::builder();

    if let Some(salt) = config.salt {
        builder = builder.salt(salt);
    }

    if let Some(length) = config.length {
        builder = builder.length(length);
    }

    if let Some(alphabet) = config.alphabet {
        builder = builder.alphabet(alphabet);
    }

    match builder.build() {
        Ok(harsh) => harsh,
        Err(e) => {
            println!("Error: {}", e);
            Harsh::default()
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Globals {
            harsh: get_harsh(),
        })
        .mount("/", routes![get, post, nav])
        .attach(Template::fairing())
}
