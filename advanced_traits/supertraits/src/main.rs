/*
 *
 * Just wanna do something simple with super traits
 *
 * x implements y which does some kind of println thing, and then my method would take require type x, but it has to implement y aswell
 *
 */

fn main() {
    say_hello_through_X(X {});
}

fn say_hello_through_X(x: X) {
    x.say_hello();
}

struct X {}

trait Y {
    fn say_hello(&self);
}

impl Y for X {
    fn say_hello(&self) {
        println!("hello");
    }
}

//i thought this was something new, but it was stuff i already knew
