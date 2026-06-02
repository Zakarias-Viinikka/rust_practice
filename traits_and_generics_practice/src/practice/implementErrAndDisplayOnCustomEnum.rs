/*Exercise: Giving your custom error a human-readable message and cause tracing

You have your error enum. Now you make it behave like a proper error.

You implement Display so you can print it with println!("{}", err). You write a match on your variants and output a clear message for each one — "couldn't read the file", "the number was invalid", etc.

You implement Error so you can trace back to the original cause. The source() method returns the error that triggered this one. If MathError was caused by ParseError which was caused by a file error, each source() points one level deeper. This lets you follow the whole chain down to the root problem.

Both together make your error enum a full error type that Rust tooling understands.*/
