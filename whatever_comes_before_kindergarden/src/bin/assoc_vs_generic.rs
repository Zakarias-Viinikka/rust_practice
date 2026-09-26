use std::marker::PhantomData;

fn main() {
    let the_normal_way = do_something::<i32>();

    let not_normal_way_struct = MyStruct::<bool> {
        _marker: PhantomData,
    };

    let the_not_normal_way = not_normal_way_struct.do_something_through_trait::<bool>();

    println!("the_normal_way: {}", the_normal_way);
    println!("the_not_normal_way: {}", the_not_normal_way);
}

fn do_something<T>() -> String {
    "takes_a_t".into()
}

// --
// --
// --

struct MyStruct<T> {
    _marker: PhantomData<T>,
}

trait DoSomethingTrait {
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
}

//structy_wucty.do_something()
