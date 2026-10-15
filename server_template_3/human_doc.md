**server_protocol**

The single place that declares what goes over the wire. Message, Response, Request, UnbuiltRequest, UnbuiltResponse, and the response shapes (like GetEntireTableOut). Also re-exports the pieces of z_db's protocol crate that the client needs to handle payloads: DbError, Row, Col, Base64Bytes, Convert, ok_serialized. The client never imports z_db directly.

**server_db_schema**

The table definitions. `all_tables()` returns a `Vec<DbTable>` where each DbTable is a table name plus its `Vec<ColumnDef>`. The server uses it on startup to create every table that doesn't exist yet.

**queue**

The queue manager handles adding to the queue. The queue executor (in server, not in this crate) executes on the queue.

**server**

The server does websocket stuff. The queue manager and executor handle the requests. The server main just sets up the websocket stuff and constructs them.

**how_to_talk_to_server**

How the client talks to the server.

**incoming_responses**

So the client doesn't have to deal with incoming responses itself.

**test_web_client**

A web client for testing the server.