#![allow(warnings)]
use std::{clone, marker::PhantomData};

use protocol::row_col::Col;

fn main() {
    method1::<i32>();
    method2::<String>();
}

#[derive(Clone)]
struct Struct<T> {
    _maker: PhantomData<T>,
}

fn method1<T>()
where
    T: IsSame<i32>,
{
    println!("hello");
}

fn method2<T>()
where
    T: IsSame<String>,
{
    println!("hello");
}

trait IsSame<T> {}
impl<T> IsSame<T> for T {}
