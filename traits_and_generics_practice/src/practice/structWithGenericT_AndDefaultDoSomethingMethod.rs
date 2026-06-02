/*i was thinking of creating a struct that holds a generic type and has a default implementation for printing the value if it's a string or a number (the number part with the help of "if the value has trait") or something like that. otherwise requires an custom implementation you have to make yourself. that's just the first idea i had*/

/*notes after finishing: i got the intial thing to print "things that have the display trait" pretty easily but i wanted to add an implementation manually for a list of numbers just to try and make it work. got kinda messy but at least it runs now? */
#![allow(warnings)]
use std::fmt::Display;

pub struct Struct<T> {
    pub value: T,
}

pub trait PrintOrSomething {
    fn printOrSomething(&self);
}
impl<T: Printable> PrintOrSomething for Struct<T> {
    fn printOrSomething(&self) {
        println!("{}", self.value);
    }
}

pub trait Printable: Display {}

impl Printable for String {}
impl Printable for i32 {}
