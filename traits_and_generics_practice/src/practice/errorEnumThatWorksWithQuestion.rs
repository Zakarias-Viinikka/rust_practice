/*You have a function that does multiple things — read a file, parse numbers, do math. Each step can fail in its own way. Instead of handling each error right there (like .map_err everywhere), you make your own list of problems (an enum).

You teach Rust how to turn each specific error into your error list by implementing From. You do this for each error type you expect (like From<io::Error> and From<ParseIntError>). Inside each From, you just take the error and wrap it in the right variant of your enum. Rust handles the rest.

Why bother? Because now you can use ? everywhere in your function. When something fails, ? sees the error, finds your From implementation, converts it to your custom enum automatically, and returns it. You don't have to manually convert errors at every step.

Your function returns Result<SomeValue, YourErrorEnum>. The caller gets one clean error type to deal with, even though many different things could've gone wrong inside.*/
