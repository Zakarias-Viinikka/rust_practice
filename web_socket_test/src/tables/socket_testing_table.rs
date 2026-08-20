use shared_types::create_table;

pub const COLUMN_NAME: &str = "happy_col_name";

pub const COLUMN_DEFINITIONS: Vec<create_table::ColumnDef> = vec![
    create_table::id_column(),
    create_table::default_col(create_table::ColumnType::Text, COLUMN_NAME),
];
