mod black_magic;
mod black_magic_read;
mod create_sql_statements;
mod db_table;
mod utils;

use serde_wasm_bindgen::to_value;
use sqlite_wasm_rs::{self as ffi};
use wasm_bindgen::prelude::*;

use crate::db_table::*;
#[wasm_bindgen]
pub struct LiveForever {
    db_conn: *mut ffi::sqlite3,
    table: Table,
}

#[wasm_bindgen]
impl LiveForever {
    pub async fn new(conn_name: String) -> Result<LiveForever, JsValue> {
        let db_conn = black_magic::create_local_db_connection(&conn_name)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let table = Table {
            table_name: "content".to_string(),
            columns: vec![
                Column {
                    column_name: "textblock".to_string(),
                    column_type: ColumnType::Text,
                },
                Column {
                    column_name: "metadata".to_string(),
                    column_type: ColumnType::Text,
                },
            ],
        };
        black_magic::create_table(db_conn, &table)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(LiveForever {
            db_conn: db_conn,
            table,
        })
    }

    /*pub fn read_from_db(
        db: *mut ffi::sqlite3,
        table_name: String,
        identifier: String,
    ) */
    pub async fn get_data(
        &self,
        table_name: String,
        arguments: String,
        columns_to_read: Vec<String>,
    ) -> Result<JsValue, JsValue> {
        let result =
            black_magic_read::read_from_db(self.db_conn, table_name, arguments, columns_to_read)
                .map_err(|e| JsValue::from(e.to_string()))?;
        let result =
            serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from(e.to_string()))?;
        Ok(result) //serde-wasm-bindgen = "0.6.5"
    }

    //console.log(state.change_data("new data"));
    pub async fn insert_data(&mut self, text: String, meta_data: String) -> Result<(), JsValue> {
        let values = vec![text, meta_data];
        black_magic::insert_into_table(self.db_conn, &self.table, values)
            .map_err(|e| JsValue::from(e.to_string()))?;
        Ok(())
    }

    pub async fn drop_table(&self) -> Result<(), JsValue> {
        black_magic::drop_table(self.db_conn, &self.table.table_name)?;
        Ok(())
    }

    //pub fn table_shape(db_conn: *mut ffi::sqlite3, table_name: &str) -> Result<Vec<String>, JsValue> {
    pub async fn check_table(&self, table_name: &str) -> Result<Vec<String>, JsValue> {
        black_magic::table_shape(self.db_conn, table_name)
    }
}

// https://wasm-bindgen.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
