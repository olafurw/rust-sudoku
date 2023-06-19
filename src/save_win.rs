use rusqlite::{params, Connection};

pub fn load(key: &str) -> Option<String> {
    let db = Connection::open("save.db").unwrap();
    let mut stmt = db.prepare("SELECT value FROM save WHERE key = ?1").unwrap();
    let mut rows = stmt.query(params![key]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let value: String = row.get(0).unwrap();
    Some(value)
}

pub fn save(key: &str, value: &str) {
    let db = Connection::open("save.db").unwrap();
    db.execute(
        "REPLACE INTO save (key, value) VALUES (?1, ?2)",
        params![key, value],
    )
    .unwrap();
}
