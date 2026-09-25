use protocol::new_table;
use protocol::row_col;

pub const TABLE_NAME: &str = "greatest_table_ever";

pub const COL_NAME: &str = "the_greatest_column_ever";

pub enum TheGreatestColumnEver {}

pub fn table_preparer() -> Vec<new_table::ColumnDef> {
    vec![
        new_table::id_column(),
        new_table::default_col(new_table::ColumnType::Text, COL_NAME),
    ]
}

pub fn make_new_row(text: String) -> row_col::Row {
    row_col::Row {
        cols: vec![row_col::Col::Text(text)],
    }
}
