the entire point of this project is to answe the question:

I have a struct with a field that has an enum with fields such as 
TEXT,
BLOB,
NUMBER

and i have another field with just a string that is meant to be parsed to one of those things

if i know that I want to parse it to T

how do i express

parse_to<T>()
---
above is old lies. this is new truth:

tldr.

try_from will turn my string into num,vec<u8>, string o whatever else.

the phantom t says what .into() is suppoed to try to convert into.

and that's that.

the caller gives a concrete T the thing will return either err or a concrete T

but the T is part of the struct defition so it would be provided by default or made to be provided by default
