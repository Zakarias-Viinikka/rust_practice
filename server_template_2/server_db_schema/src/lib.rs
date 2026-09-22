use protocol::new_table::{ColumnDef, ColumnType, id_column, not_null_col};

pub struct DbTable {
    pub table_name: String,
    pub columns: Vec<ColumnDef>,
}

pub fn all_tables() -> Vec<DbTable> {
    vec![
        DbTable {
            table_name: "simple_table".to_string(),
            columns: simple_table_columns(),
        },
        DbTable {
            table_name: "cars".to_string(),
            columns: cars_columns(),
        },
    ]
}

fn simple_table_columns() -> Vec<ColumnDef> {
    vec![
        id_column(),
        not_null_col(ColumnType::Text, "animal"),
        not_null_col(ColumnType::Text, "color"),
    ]
}

fn cars_columns() -> Vec<ColumnDef> {
    vec![
        id_column(),
        not_null_col(ColumnType::Text, "make"),
        not_null_col(ColumnType::Integer, "year"),
    ]
}
