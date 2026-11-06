use std::path::Path;
use std::sync::Mutex;

use rusqlite::Connection;

use super::migrations::run_migrations;

pub type DbConnection = Mutex<Connection>;

pub fn open_connection(db_path: &Path) -> anyhow::Result<Connection> {
    let mut conn = Connection::open(db_path)?;
    conn.pragma_update(None, "foreign_keys", true)?;
    run_migrations(&mut conn)?;
    Ok(conn)
}
