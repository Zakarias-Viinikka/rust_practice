fn main() {
    magic_box(number_5, 3);
}

fn magic_box(f: fn(i32) -> i32, other_number: i32) {
    println!("I can turn any number into 5.\n Here. look!");
    println!("Your number was: {}", other_number);
    println!("And now it has become {}", f(other_number))
}

fn number_5(useless_number: i32) -> i32 {
    5
}
