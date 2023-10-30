use rusqlite::{Connection, Result};
// Establishes a connection to the database
pub fn establish_connection() -> Result<Connection> {
    Connection::open("cli_app_db.sqlite3")
}
