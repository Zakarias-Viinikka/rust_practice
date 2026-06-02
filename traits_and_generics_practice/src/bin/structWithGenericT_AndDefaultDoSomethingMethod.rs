#![allow(warnings)]
#[path = "../practice/mod.rs"]
mod practice;
use std::fmt::Display;

use num_traits::Num;
use practice::structWithGenericT_AndDefaultDoSomethingMethod as swgtadsm;

//cargo run -q
use swgtadsm::PrintOrSomething;
use swgtadsm::Struct;
fn main() {
    let x = Struct { value: 3 };
    x.printOrSomething();

    let y = Struct { value: [7, 2, 5] };
    y.printOrSomething();
    let z = Struct {
        value: [13, 23, 5, 5, 665],
    };
    z.printOrSomething();
}

//i forgot to use vec so i ended up doing the cont N stuff to make the array work.
impl<T: Num + std::fmt::Display, const N: usize> PrintOrSomething for Struct<[T; N]> {
    fn printOrSomething(&self) {
        let mut string: String = "".to_string();
        for i in &self.value {
            string = format!("{}, {}", string, i);
        }
        println!("{}", string);
    }
}
