use rusqlite::{params, Connection};

pub fn save(value: &str) {
    let db = Connection::open("save.db").unwrap();
    db.execute(
        "REPLACE INTO save (key, value) VALUES ('save2', ?1)",
        params![value],
    )
    .unwrap();
}
