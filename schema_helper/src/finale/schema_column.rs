use std::marker::PhantomData;

use protocol::{
    new_table,
    row_col::{self, Col},
};

/*pub struct SchemaTable {
    table_name: &'static str,
}*/

pub struct SchemaColumn<T> {
    pub name: &'static str,
    pub type_of_col: &'static row_col::Col,
    pub _marker: PhantomData<T>,
}

pub trait DestructDbReturnCol {
    type Output;
    fn destruct_db_col(&self, db_col: Col) -> Result<Self::Output, String>;
}

impl DestructDbReturnCol for SchemaColumn<i64> {
    type Output = i64;
    fn destruct_db_col(&self, db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Integer(value_as_i64) => Ok(value_as_i64),
            _ => Err("illegal".into()),
        }
    }
}
