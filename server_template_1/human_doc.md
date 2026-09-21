**server_protocol**

Holds all the requests and responses the server takes. As well as helpers for how to build a request or a response, and helpers for serializing deserializing easily.

The goal is that the protocol makes it easy for the server and the caller side to parse messages, and all they have to do is match the message variant that the payload carries.

**server_talker**

Is in charge of actually talking to the server.

The point of server_talker is that the client builds a request. And then does rx tx with the server_talker and just says "this is my request. deal with the rest."

**router_for_responses_coming_to_client**

However, the client doesn't actually talk to the server_talker directly.

The Router is the part that handles connecting messages to responses since requests and responses aren't tied together when talking to a websocket, but the caller expects a response for a request.

That way the client can say "I have this request. Deal with it router" and then the router hands back a rx receiver that the client will use for receiving a response.

**mock_server**

Is dumb and can be ignored in this proj.

**test_web_client**

Is "dumb" because the server_talker and router handle all the stuff for dealing with the server.

And because of a helper method all it has to do is call `setup_server()` which returns a router it talks to.
