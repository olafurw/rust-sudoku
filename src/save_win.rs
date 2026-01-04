use rusqlite::{params, Connection, OpenFlags};

pub fn load(key: &str) -> Option<String> {
    let db = get_connection();
    let mut stmt = db.prepare("SELECT value FROM save WHERE key = ?1").unwrap();

    let mut rows = stmt.query(params![key]).unwrap();
    let row = rows.next();
    if row.is_err() {
        return None;
    }

    let row = row.unwrap()?;
    let value: String = row.get(0).unwrap();
    Some(value)
}

pub fn save(key: &str, value: &str) {
    let db = get_connection();
    db.execute(
        "REPLACE INTO save (key, value) VALUES (?1, ?2)",
        params![key, value],
    )
    .unwrap();
}

fn migrate_db(db: &Connection) {
    let mut stmt = db
        .prepare("CREATE TABLE IF NOT EXISTS save (key VARCHAR(255) PRIMARY KEY, value LONGTEXT);")
        .unwrap();
    stmt.execute(params![]).unwrap();
}

fn get_connection() -> Connection {
    let conn = Connection::open_with_flags("save.db", get_database_flags())
        .expect("Could not open database.");

    migrate_db(&conn);

    return conn;
}

fn get_database_flags() -> OpenFlags {
    let mut db_flags = OpenFlags::empty();

    db_flags.insert(OpenFlags::SQLITE_OPEN_READ_WRITE);
    db_flags.insert(OpenFlags::SQLITE_OPEN_CREATE);
    db_flags.insert(OpenFlags::SQLITE_OPEN_FULL_MUTEX);
    db_flags.insert(OpenFlags::SQLITE_OPEN_NOFOLLOW);
    db_flags.insert(OpenFlags::SQLITE_OPEN_PRIVATE_CACHE);

    db_flags
}
