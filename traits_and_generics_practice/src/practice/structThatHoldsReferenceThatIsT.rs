//struct MyStruct<'a, T> { value: &'a T }
/*
You have a struct that holds a reference to a value rather than owning the value itself. The value lives somewhere else — the struct just points to it.

The struct is generic over both the type T and the lifetime 'a (how long the reference is valid). This means you can use the same struct to look at an i32, a String, or any other type, as long as the borrowed value is still alive.

Give it a method or two that do something useful with the borrowed value — like printing it (needs T: Display), comparing it to another reference of the same type (needs T: PartialEq), or determining which of two references is larger (needs T: PartialOrd).


*/
