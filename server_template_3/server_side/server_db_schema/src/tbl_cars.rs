use protocol::new_table::{ColumnDef, ColumnType, id_column, not_null_col};
use protocol::schema_helper::{SchemaColumn, TypeOfCol};
use std::marker::PhantomData;

pub const TABLE_NAME: &str = "cars";

pub const MAKE: SchemaColumn<String> = SchemaColumn::<String> {
    name: "make",
    type_of_col: &TypeOfCol::Text,
    can_be_null: false,
    _marker: PhantomData,
};

pub const YEAR: SchemaColumn<i64> = SchemaColumn::<i64> {
    name: "year",
    type_of_col: &TypeOfCol::Integer,
    can_be_null: false,
    _marker: PhantomData,
};

pub fn columns() -> Vec<ColumnDef> {
    vec![
        id_column(),
        not_null_col(ColumnType::Text, MAKE.name),
        not_null_col(ColumnType::Integer, YEAR.name),
    ]
}