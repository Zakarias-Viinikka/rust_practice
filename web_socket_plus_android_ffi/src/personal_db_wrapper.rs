use crate::tables::socket_testing_table;
use db_weebee::mascot::{self, LiveForever};
use db_weebee::{black_magic, black_magic_read};
use shared_types::db_error::DbError;
use shared_types::{data_structures, table_row};

const FILEPATH: &str = "test_db.sqlite";
const TABLE_NAME: &str = "socket_testing_table";

pub fn new_liver() -> mascot::LiveForever {
    mascot::LiveForever::new(FILEPATH).unwrap()
}

pub fn create_table_if_not_exist(liver: &LiveForever) -> Result<(), DbError> {
    let columns = socket_testing_table::column_definitions();
    let conn = &liver.db_conn;
    black_magic::create_table(conn, TABLE_NAME, columns)
}

pub fn create_data_if_not_exist(liver: &LiveForever) -> Result<(), DbError> {
    if !any_rows_exist(liver)? {
        let column_name = socket_testing_table::COLUMN_NAME;

        let mut stuff_to_insert = Vec::new();
        stuff_to_insert.push(data_structures::ColumnValue {
            column_name: column_name.to_string(),
            value: table_row::Col::Text("text1".to_string()),
        });
        stuff_to_insert.push(data_structures::ColumnValue {
            column_name: column_name.to_string(),
            value: table_row::Col::Text("text2".to_string()),
        });
        stuff_to_insert.push(data_structures::ColumnValue {
            column_name: column_name.to_string(),
            value: table_row::Col::Text("text3".to_string()),
        });

        for inserty_werty in stuff_to_insert {
            black_magic::insert_into_table(&liver.db_conn, TABLE_NAME, vec![inserty_werty])?;
        }
    }
    Ok(())
}

fn any_rows_exist(liver: &LiveForever) -> Result<bool, DbError> {
    let conn = &liver.db_conn;
    let ctx = data_structures::GetDataIn {
        table_name: TABLE_NAME.to_string(),
        arguments: vec![data_structures::SelectArgument::All],
        columns_to_read: Vec::new(),
    };
    let rows = black_magic_read::read_from_db(conn, &ctx)?;
    Ok(!rows.is_empty())
}
