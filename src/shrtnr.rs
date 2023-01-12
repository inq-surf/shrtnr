const DB_PATH: &str = "data/db";

pub fn add(url: &str) -> Result<u64, String> {
    let db = match sled::open(DB_PATH) {
        Ok(db) => db,
        Err(e) => {
            return Err(e.to_string());
        },
    };

    let id = match db.generate_id() {
        Ok(id) => id,
        Err(e) => {
            return Err(e.to_string());
        },
    };

    match db.insert(id.to_be_bytes(), url.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            return Err(e.to_string());
        },
    };

    Ok(id)
}

pub fn get(id: &u64) -> Result<String, String> {
    let db = match sled::open(DB_PATH) {
        Ok(db) => db,
        Err(e) => {
            return Err(e.to_string());
        },
    };

    let value = match db.get(id.to_be_bytes()) {
        Ok(Some(value)) => value,
        Ok(None) => {
            return Err(format!("No value found for {}", id));
        },
        Err(e) => {
            return Err(e.to_string());
        },
    };

    let url = match std::str::from_utf8(&value) {
        Ok(url) => url,
        Err(e) => {
            return Err(e.to_string());
        },
    };

    Ok(url.to_string())
}
