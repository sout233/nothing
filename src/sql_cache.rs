use rusqlite::{Connection, Result};

fn cached() {
    let conn = Connection::open("filesearch.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
        id INTEGER PRIMARY KEY,
        path TEXT NOT NULL,
        size INTEGER NOT NULL,
        modified INTEGER NOT NULL
    )",
        [],
    );

    let tx = conn.transaction().unwrap();
    for file_info in &file_map {
        tx.execute(
            "INSERT INTO files (path, size, modified) VALUES (?1, ?2, ?3)",
            &[
                &file_info.path.to_string_lossy(),
                &file_info.size,
                &file_info.modified,
            ],
        );
    }
    tx.commit();
}
