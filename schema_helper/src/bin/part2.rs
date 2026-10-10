#![allow(warnings)]
use protocol::{
    row_col::{Col, StructRepresentingNull},
    serialization::Convert,
};

fn main() {}

//trying to solve this:
//
// taking an enum and a T then turning enum to T
//
//
// SpecificEnum1(String)
//

trait ConverterBelt {
    type Output;
    fn destruct_db_col<F>(db_col: Col) -> Result<Self::Output, String>;
}

// ??? !!! |||

// All the implementations down here

// ??? !!! |||

impl ConverterBelt for String {
    type Output = String;
    fn destruct_db_col<F>(db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Text(value_as_string) => Ok(value_as_string),
            _ => Err("illegal".into()),
        }
    }
}

impl ConverterBelt for i64 {
    type Output = i64;
    fn destruct_db_col<F>(db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Integer(value_as_i64) => Ok(value_as_i64),
            _ => Err("illegal".into()),
        }
    }
}

impl ConverterBelt for f64 {
    type Output = f64;
    fn destruct_db_col<F>(db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Real(value_as_f64) => Ok(value_as_f64),
            _ => Err("illegal".into()),
        }
    }
}

impl ConverterBelt for StructRepresentingNull {
    type Output = StructRepresentingNull;
    fn destruct_db_col<F>(db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Null(_) => Ok(StructRepresentingNull {}),
            _ => Err("illegal".into()),
        }
    }
}

impl ConverterBelt for Vec<u8> {
    type Output = Vec<u8>;
    fn destruct_db_col<F>(db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Blob(value_as_bytes) => Ok(value_as_bytes),
            _ => Err("illegal".into()),
        }
    }
}

/*
pub enum Col {
    Null( /*  */ ),
    Integer( /* … */ ),
    Real( /* … */ ),
    Text( /* … */ ),
    Blob( /* … */ ),
}
 */

/*

match db_col {
    Col::Text(v) => v.try_into::<F>(),
    _ => todo!(),
}
 */
