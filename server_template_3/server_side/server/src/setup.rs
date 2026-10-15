use db_wrapper::mascot::LiveForever;
use protocol::payload::*;
use protocol::row_col::Col;
use server_db_schema::all_tables;

const DB_FILENAME: &str = "test_db.sqlite";

pub fn open_db() -> LiveForever {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), DB_FILENAME);
    println!("[server] opening db at {}", path);
    match LiveForever::new(&path) {
        Ok(liver) => liver,
        Err(e) => {
            eprintln!("[server] could not open db: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn create_all_tables(liver: &LiveForever) {
    for table in all_tables() {
        let name = table.table_name.clone();
        match liver.create_table(CreateTableIn {
            table_name: name.clone(),
            columns: table.columns,
        }) {
            Ok(()) => println!("[server] created table {}", name),
            Err(e) => println!("[server] table {} not created: {}", name, e),
        }
    }
}

fn table_is_empty(liver: &LiveForever, table_name: &str) -> Result<bool, String> {
    liver
        .get_data(GetDataIn {
            table_name: table_name.to_string(),
            arguments: SelectArguments::Single(SelectArgument::All),
            columns_to_read: Vec::new(),
        })
        .map(|out| out.rows.is_empty())
        .map_err(|e| e.to_string())
}

pub fn seed_tables(liver: &LiveForever) {
    seed_if_empty(
        liver,
        "simple_table",
        vec![
            vec![
                ("animal".to_string(), Col::Text("cat".to_string())),
                ("color".to_string(), Col::Text("black".to_string())),
            ],
            vec![
                ("animal".to_string(), Col::Text("dog".to_string())),
                ("color".to_string(), Col::Text("brown".to_string())),
            ],
        ],
    );

    seed_if_empty(
        liver,
        "cars",
        vec![
            vec![
                ("make".to_string(), Col::Text("Toyota".to_string())),
                ("year".to_string(), Col::Integer(2020)),
            ],
            vec![
                ("make".to_string(), Col::Text("Honda".to_string())),
                ("year".to_string(), Col::Integer(2018)),
            ],
        ],
    );
}

fn seed_if_empty(liver: &LiveForever, table_name: &str, rows: Vec<Vec<(String, Col)>>) {
    match table_is_empty(liver, table_name) {
        Ok(true) => {}
        Ok(false) => {
            println!("[server] {} already has rows, not seeding", table_name);
            return;
        }
        Err(e) => {
            println!("[server] could not check {} for rows: {}", table_name, e);
            return;
        }
    }

    for row in rows {
        let values = row
            .into_iter()
            .map(|(column_name, value)| ColumnValue { column_name, value })
            .collect();
        match liver.insert_data(InsertDataIn {
            table_name: table_name.to_string(),
            values,
        }) {
            Ok(()) => println!("[server] seeded a row into {}", table_name),
            Err(e) => println!("[server] could not seed {}: {}", table_name, e),
        }
    }
}