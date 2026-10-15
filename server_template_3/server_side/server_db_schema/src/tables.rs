use crate::{tbl_cars, tbl_simple_table};
use protocol::new_table::ColumnDef;

pub struct DbTable {
    pub table_name: String,
    pub columns: Vec<ColumnDef>,
}

pub fn all_tables() -> Vec<DbTable> {
    vec![
        DbTable {
            table_name: tbl_simple_table::TABLE_NAME.to_string(),
            columns: tbl_simple_table::columns(),
        },
        DbTable {
            table_name: tbl_cars::TABLE_NAME.to_string(),
            columns: tbl_cars::columns(),
        },
    ]
}