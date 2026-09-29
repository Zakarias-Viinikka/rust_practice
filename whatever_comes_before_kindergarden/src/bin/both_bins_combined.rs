#![allow(warnings)]
use std::marker::PhantomData;

use protocol::{
    row_col::{Col, StructRepresentingNull},
    serialization::Convert,
};

fn main() {
    let s = MyStruct::<i64> {
        _marker: PhantomData,
    };

    let x = s.destruct_db_col(Col::Integer(3));
    let x2 = s.destruct_db_col(Col::Text("test".into()));

    println!("{:?}", x);
    println!("{:?}", x2);
}

impl ConverterBelt for MyStruct<i64> {
    type Output = i64;
    fn destruct_db_col(&self, db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Integer(value_as_i64) => Ok(value_as_i64),
            _ => Err("illegal".into()),
        }
    }
}

// --
// --
// --

// bin 1

struct MyStruct<T> {
    _marker: PhantomData<T>,
}

/*trait DoSomethingTrait {
    type Assoc;
    fn do_something_through_trait<T>(&self) -> String
    where
        Self: DoSomethingTrait<Assoc = T>;
}

impl<T> DoSomethingTrait for MyStruct<T> {
    type Assoc = T;
    fn do_something_through_trait<F>(&self) -> String
    where
        Self: DoSomethingTrait<Assoc = F>,
    {
        "takes_a_t_through_trait".into()
    }
}*/

// --
// --
// --

trait ConverterBelt {
    type Output;
    fn destruct_db_col(&self, db_col: Col) -> Result<Self::Output, String>;
}

// ??? !!! |||

// All the implementations down here

// ??? !!! |||

impl ConverterBelt for String {
    type Output = String;
    fn destruct_db_col(&self, db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Text(value_as_string) => Ok(value_as_string),
            _ => Err("illegal".into()),
        }
    }
}

impl ConverterBelt for i64 {
    type Output = i64;
    fn destruct_db_col(&self, db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Integer(value_as_i64) => Ok(value_as_i64),
            _ => Err("illegal".into()),
        }
    }
}

impl ConverterBelt for f64 {
    type Output = f64;
    fn destruct_db_col(&self, db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Real(value_as_f64) => Ok(value_as_f64),
            _ => Err("illegal".into()),
        }
    }
}

impl ConverterBelt for StructRepresentingNull {
    type Output = StructRepresentingNull;
    fn destruct_db_col(&self, db_col: Col) -> Result<Self::Output, String> {
        match db_col {
            Col::Null(_) => Ok(StructRepresentingNull {}),
            _ => Err("illegal".into()),
        }
    }
}

impl ConverterBelt for Vec<u8> {
    type Output = Vec<u8>;
    fn destruct_db_col(&self, db_col: Col) -> Result<Self::Output, String> {
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
