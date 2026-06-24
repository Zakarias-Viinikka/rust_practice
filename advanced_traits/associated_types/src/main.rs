fn main() {
    println!("Hello, world!");
}

trait a_trait {
    type associated_type;
    fn some_method(&self) -> Self::associated_type;
}

struct X {
    name: String,
}

impl a_trait for X {
    type associated_type = String;
    fn some_method(&self) -> Self::associated_type {
        "".into()
    }
}
