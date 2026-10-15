# server_template_3 — AI stored context

Written by AI for AI. Read this before touching the project. Update the
relevant section when its content changes and bump that section's date. If
code and this file disagree, the code wins and this file should be
corrected.

## What this is — 2026-10-15

A practice workspace for a websocket server and its browser client. The
server accepts websocket connections on port 3000, puts each incoming
request into a queue, runs the queued requests against a sqlite database,
and sends each answer back over the same connection. The browser side
connects to it and shows responses. This is a stepping stone toward
zakken_server, which is a separate project and is not changed by this one.

## Workspace layout — 2026-10-15

One root cargo workspace. The root Cargo.toml lists every member. Folders:

- shared/error_stuff — error type used by both sides.
- shared/server_protocol — Message, Response, Request, UnbuiltRequest and
  payload shapes. Both sides import it.
- server_side/server — the websocket server binary (main.rs wiring,
  setup.rs database, socket.rs connection loop, queue_executor.rs
  background loop, requests.rs handle_request).
- server_side/server_db_schema — table definitions, one file per table.
- server_side/queue — Job, Priority, QueueManager. No database knowledge.
- client_side/how_to_talk_to_server — ServerTalker: opens the websocket and
  gives back channel handles.
- client_side/incoming_responses — ResponseRouter: matches each response to
  the request that asked for it.
- client_side/test_web_client — leptos CSR page for testing. Runs only in
  the browser, through trunk.

## Request flow — 2026-10-15

Each websocket message is parsed in socket.rs and turned into a Job with a
reply channel. Ping is Priority::Important; everything else is Normal. The
QueueManager sends the job into one of two tokio channels. The executor loop
in queue_executor.rs takes from the important channel first (biased select),
calls requests::handle_request against the database, and sends the result
back through the job's reply channel. socket.rs waits on that reply and
writes it to the socket.

The queue crate never names the server's request code. The server passes
the database and handler into the executor, which keeps the crates
acyclic: server depends on queue, not the other way around.

## Database — 2026-10-15

setup.rs opens the sqlite file next to the crate, creates every table from
server_db_schema::all_tables() if missing, and seeds starting rows into
simple_table and cars if they are empty. Only those two test tables exist.
The client's real tables are not in this server yet.

## Dependencies — 2026-10-15

db_wrapper and protocol are path dependencies into /home/zakke/ProgStuff/z_db.
They are absolute paths, so they break if z_db moves. server_protocol's
protocol dependency is the same local path. Do not switch it back to the
GitHub git dependency: that made two copies of the protocol types, and they
clashed.

## Commands — 2026-10-15

- c c — cargo check the workspace (default member is server).
- c r — run the server from the root. From inside client_side it runs trunk
  instead.
- c web — always runs trunk serve for test_web_client.
- local_cargo.sh is the per-directory script `c` finds by walking up.

## Gotchas — 2026-10-15

test_web_client panics if run as a native binary. It is wasm only, so run it
with trunk.

Folder moves can't be done through zap. Use the terminal for mv and mkdir.

## What's left — 2026-10-15

- The server's test tables are not the client's real tables. The schema
  does not match client_table_blueprints yet.
- human_doc.md is out of date for the queue crate and gets rewritten together
  at the end.
- The Important priority is used only for Ping.
- The combination with zakken_server is not started.
