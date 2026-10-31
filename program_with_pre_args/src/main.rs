fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("hello");
    } else {
        println!("u ran with {}", args.join(" "));
    }
}