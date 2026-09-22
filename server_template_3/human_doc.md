**server_protocol**

The single place that declares what goes over the wire. Message, Response,
Request, UnbuiltRequest, UnbuiltResponse, and the response shapes (like
GetEntireTableOut). Also re-exports the pieces of z_db's protocol crate that
the client needs to handle payloads — DbError, Row, Col, Base64Bytes, Convert,
ok_serialized — so the client never imports z_db directly.

**server_db_schema**

The table definitions. `all_tables()` returns a `Vec<DbTable>` where each
DbTable is a table name plus its `Vec<ColumnDef>`. The server uses it on
startup to create every table that doesn't exist yet.

**server**

The axum websocket server. On startup it opens the sqlite file next to the
crate, creates tables from server_db_schema, and seeds two rows into each if
the table is empty. On a request it reads/writes through z_db (via db_wrapper)
and replies over the socket. All DB access goes through a `Mutex<LiveForever>`
so it can be shared across websocket tasks.

**server_talker**

Runs in the browser. Holds the websocket. Gives you three handles: a tx for
outgoing messages, an rx for incoming responses, and an rx for connection
state. It doesn't match responses to requests — it just forwards.

**router_for_responses_coming_to_client**

Sits between the client code and server_talker. Requests and responses aren't
tied together over a websocket, but the caller expects a reply to its request.
The router generates a message_id, hands it to a callback you provide (so you
build the message with the id already in it), sends it, and gives you back a
receiver for the response. When a response comes in, the router looks up the id
and delivers it to whoever is waiting.

**test_web_client**

A leptos CSR page. Connect button, a text field where you type a table name,
a Send button, and a box in the middle of the page showing the response. It
only talks to the router.
