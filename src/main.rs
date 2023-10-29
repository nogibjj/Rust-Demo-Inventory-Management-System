pub mod models;
pub mod db;
pub mod cli;

fn main() {
    let conn = db::conn::establish_connection().expect("Failed to establish connection.");

    db::crud::initialize_database(&conn).expect("Failed to initialize database.");

    cli::commands::run_cli(&conn);
}
