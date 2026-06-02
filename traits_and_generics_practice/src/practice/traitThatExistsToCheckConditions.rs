/* how about a custom trait which had the single purpose of basically letting you check if something fulfilled specific conditions?

but trait has go to do with implementations so i feel like that's just better to use a method called "does this thing fulfill all these conditions" and the parameter would require thing.fulfillconditions to be true.
*/

/*
so if you wanted conditional stuff it would be more like

"this things has 3 methods that return these 3 different things and i only need specifically those 3 things and the only way you can send in a parameter is something the compiler decided fulfilled the conditions for implementing those 3 things"

and to clarify. the only way the compiler would let me use the type passed in is by calling those traits or whatever they're called.
 */
