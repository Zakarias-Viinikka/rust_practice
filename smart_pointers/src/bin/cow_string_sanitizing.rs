//i wanted to use the cow smart pointer to sanitize user input. like if the user uses specific symbol or symbol combinations. i make the caller of "get user input" get a new string, but otherwise the cow pointer would return a reference.
/* if i didn't use cow then i would be making a new string every time

cuz i would have something like

let raw = getUserInputRaw()

and then send it into sanitizer and sanitizer would otherwise make a clone every time?

also maybe wanna do this in another project. but just making a simple enum that has 2 variants that both store strings just to keep track of whether the string is of a sanitized or unsanitized type so my compiler can check whether im forgetting to sanitize stuff if every want to do anything with it.

an enum would force me to handle different situations so it's better if i make 2 different enum's with 1 variant or just 2 different structs.
*/

fn main() {}
