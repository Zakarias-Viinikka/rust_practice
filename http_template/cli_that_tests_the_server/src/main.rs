fn main() {
    match cli_that_tests_the_server::call_server("hello from client") {
        Ok(reply) => println!("reply: {}", reply),
        Err(e) => println!("error: {}", e),
    }
}
