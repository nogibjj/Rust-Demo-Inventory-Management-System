use rusqlite::{Connection, Result};
// Establishes a connection to the database
pub fn establish_connection() -> Result<Connection> {
    Connection::open("cli_app_db.sqlite3")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_establish_connection() {
        let connection_result = establish_connection();
        assert!(connection_result.is_ok(), "Failed to establish connection to the database.");
        
    }    
}