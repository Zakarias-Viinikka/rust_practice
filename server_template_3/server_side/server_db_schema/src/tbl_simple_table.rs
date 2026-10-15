use protocol::new_table::{ColumnDef, ColumnType, id_column, not_null_col};
use protocol::schema_helper::{SchemaColumn, TypeOfCol};
use std::marker::PhantomData;

pub const TABLE_NAME: &str = "simple_table";

pub const ANIMAL: SchemaColumn<String> = SchemaColumn::<String> {
    name: "animal",
    type_of_col: &TypeOfCol::Text,
    can_be_null: false,
    _marker: PhantomData,
};

pub const COLOR: SchemaColumn<String> = SchemaColumn::<String> {
    name: "color",
    type_of_col: &TypeOfCol::Text,
    can_be_null: false,
    _marker: PhantomData,
};

pub fn columns() -> Vec<ColumnDef> {
    vec![
        id_column(),
        not_null_col(ColumnType::Text, ANIMAL.name),
        not_null_col(ColumnType::Text, COLOR.name),
    ]
}