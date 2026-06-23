#![allow(unused)]
use rusqlite::Connection;
use std::panic;
mod get_conn;
use get_conn::get_connection;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_sqlite_database");
    println!("Opening: {path}");
    let conn = get_connection(path).unwrap_or_else(|e| panic!("couldn't connect to database: {e}"));

    let list_of_items_from_table = get_table_wable_rows(conn);
    match list_of_items_from_table {
        Ok(list) => {
            for item in list {
                println!("animal is: {}, and color is: {}", item.animal, item.color);
            }
        }
        Err(e) => {
            println!("Something went wrong :/");
            println!("{}", e);
        }
    }
}

#[derive(Debug)]
struct TableItem {
    id: i32,
    pub color: String,
    pub animal: String,
}

/*impl TableItem {
    fn new(id: i32, color: String, animal: String) -> Self {
        Self { id, color, animal }
    }
}*/

fn get_table_wable_rows(conn: Connection) -> Result<Vec<TableItem>, rusqlite::Error> {
    let query = "SELECT id, color, animal FROM table_wable"; //WHERE id = 1
    //idk why u gotta do this. think it just makes stuff faster or something?
    let mut stmt = conn.prepare(query)?;
    println!("suvived the prepare statement");
    let table_rows_iter = stmt.query_map([], |row| {
        Ok(TableItem {
            id: row.get(0)?,
            color: row.get(2)?,
            animal: row.get(3)?,
        })
    })?;

    let mut list_of_animals_or_something = Vec::new();
    for item in table_rows_iter {
        list_of_animals_or_something.push(item?);
    }

    Ok(list_of_animals_or_something)
}
