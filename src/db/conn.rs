use rusqlite::{Connection, Result};

pub fn establish_connection() -> Result<Connection> {
    Connection::open("cli_app_db.sqlite3")
}
