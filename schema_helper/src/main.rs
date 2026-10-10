use std::marker::PhantomData;

use protocol::row_col::Col;
use schema_helper::finale::schema_column::{self, DestructDbReturnCol};

fn main() {
    /*
    * name
    type_of_col
    _marker
    */
    let new_col = schema_column::SchemaColumn::<i64> {
        name: "column name",
        type_of_col: &protocol::row_col::Col::Integer(5),
        _marker: PhantomData::<i64>,
    };

    let test = new_col.destruct_db_col(Col::Text("word".into()));

    println!("{:?}", test);
}
