#![allow(warnings)]
use std::{clone, marker::PhantomData};

use protocol::row_col::Col;

fn main() {
    let s1 = Struct::<i32> {
        _marker: PhantomData,
    };
    s1.method1::<i32>();

    let s2 = Struct::<String> {
        _marker: PhantomData,
    };
    s2.method2::<String>();

    let s3 = Struct::<i32> {
        _marker: PhantomData,
    };
    s3.method3();
}

#[derive(Clone)]
struct Struct<T> {
    _marker: PhantomData<T>,
}

impl<T> Struct<T> {
    fn method1<F>(&self)
    where
        T: IsSame<F>,
    {
        println!("hello");
    }
}

impl<T> Struct<T> {
    fn method2<F>(&self)
    where
        T: IsSame<F>,
    {
        println!("hello");
    }
}

impl<T> Struct<T> {
    fn method3(&self)
    where
        T: IsSame<i32>,
    {
        println!("hello");
    }
}

trait IsSame<T> {}
impl<T> IsSame<T> for T {}
