use rusqlite::Connection;
use std::error::Error;

pub fn get_connection(path: &str) -> Result<Connection, Box<dyn Error>> {
    if check_if_db_exists(path) {
        println!("connecting to database");
        Ok(try_get_connection(path)?)
    } else {
        return Err("db doesn't exist".into());
    }
}

fn try_get_connection(path: &str) -> Result<Connection, Box<dyn Error>> {
    if cfg!(test) {
        // This code runs during tests
        Err("mock error to see if get_connection handles failures properly".into())
    } else {
        Ok(Connection::open(path)?)
    }
}

fn check_if_db_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[test]
fn get_connection_fails_properly() {
    let result = get_connection("path doesn't matter");
    assert!(result.is_err());
}
