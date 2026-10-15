# server_template_3 progress

- Added the request queue in server_side/server/src/queue.rs. Socket handlers push each request into the queue and await the reply; a single worker task drains the queue and runs handle_request against the db.
- Queue shape follows actor_audition: priority and normal VecDeques (oldest first), a wake channel with capacity 1, and add_to_queue which pushes and wakes in one call.
- All requests currently go in as Priority::Normal. Priority::Important exists but nothing uses it yet.
- Fixed Cargo paths: server's db_wrapper points at z_db/db_wrapper, and server_protocol's protocol points at the local z_db/protocol (the GitHub git dependency caused two copies of the protocol types to clash).
- Moved ServerTalker and ServerTalkerHandles out of client_side/server_talker/src/lib.rs into talker.rs. lib.rs now only has mod and pub use lines. shared/server_protocol/src/lib.rs was already in that shape.
- Moved ResponseRouter out of client_side/router_for_responses_coming_to_client/src/lib.rs into core.rs. lib.rs now only has mod and pub use lines.
- Split server_db_schema into one file per table (tbl_simple_table.rs, tbl_cars.rs), each with a TABLE_NAME const and a columns() fn. DbTable and all_tables() are in tables.rs. Table definitions are unchanged.
- Reorganized into one root workspace with folders: shared/ (error_stuff, server_protocol), server_side/ (server, server_db_schema), client_side/ (server_talker, router_for_responses_coming_to_client, test_web_client). Path dependencies and local_cargo.sh updated to match. cargo check passes for the whole workspace and for the router crate.

Still open:
- server_db_schema table definitions don't yet match client_table_blueprints. This is the next item.
- human_doc.md is out of date; it gets rewritten at the end, together.
- server_db_schema tables now expose a SchemaColumn const per column (ANIMAL, COLOR, MAKE, YEAR) alongside TABLE_NAME and columns(). Column names and types are unchanged.
- Removed the unused ColumnDef import from server_side/server/src/main.rs.
- Ping requests now go in as Priority::Important. Everything else stays Normal.
- local_cargo.sh: `run` now starts the server (cargo run -p server). The client is served with `web` (trunk serve in client_side/test_web_client).
- local_cargo.sh: `run` starts trunk when the cwd is inside client_side, and starts the server (cargo run -p server) everywhere else. `web` always starts trunk.
- Queue moved out of the server into its own crate at server_side/queue. QueueManager (submit: pushes a job and wakes the executor) and QueueExecutor (spawn: drains the queues and runs a handler) are separate types. Queues holds the priority and normal VecDeques. The server passes handle_request in as the executor's handler, so the queue crate does not depend on the db.
- how_to_get_full_context.md updated to list progress.md, the three-folder layout, and the queue crate. actor_audition is no longer needed for context.
- Split server_side/server/src/main.rs into main.rs (wiring only), setup.rs (open db, create tables, seed rows), requests.rs (handle_request), and socket.rs (websocket loop and submit to the queue).
- Removed Arc and Mutex around the database handle. The queue worker's closure now owns LiveForever directly, since nothing else uses it after setup.
- Queue now uses two tokio channels (important and normal) instead of shared VecDeques and a wake channel. QueueManager sends jobs into them, QueueExecutor receives with a biased select so important jobs go first. Job and Priority are in job.rs; queue_store.rs was removed.
- Moved the "run each job against the db" step out of main.rs into server_side/server/src/worker.rs. main.rs creates the channels, the manager, and calls worker::start.
- main.rs now creates the channels, the QueueManager and the QueueExecutor itself, and spawns the executor with worker::handler(liver). worker.rs only holds the handler.
- The executor now takes the database handle and the request handler when it is created, and spawn() takes no arguments. worker.rs was removed; requests::handle_request takes the database handle as its first argument.
- Queue executor is generic over a JobHandler trait (queue/src/job_handler.rs). requests.rs has Db, a wrapper around LiveForever that implements JobHandler. The constructor takes the two receiving ends and the handler; spawn() takes nothing. worker.rs is gone.
- Removed the JobHandler trait and the Db wrapper. QueueExecutor::new takes only the two receiving ends. spawn(context, handler) takes the database and requests::handle_request, so the handler is passed where the executor runs.
- QueueExecutor now takes the context (database) and the handler in new(), and stores them as fields. spawn() takes no arguments.
- Moved the executor construction and spawn into setup.rs as setup::start_executor. main.rs no longer names requests::handle_request.
- main.rs now creates QueueManager and a QueueExecutor (via setup::new_executor), then calls queue::start_handling_queue(executor). QueueExecutor::spawn is gone; run() is private to the crate.
- The queue crate now only has Job, Priority, and QueueManager. The loop that runs jobs is worker::start in server_side/server/src/worker.rs. It calls requests::handle_request and sends each answer back through the job's reply channel. setup.rs no longer builds the executor.
- Renamed server_side/server/src/worker.rs to queue_executor.rs. Its start() function is now queue_executor::start.
- Renamed client_side/server_talker to client_side/how_to_talk_to_server and client_side/router_for_responses_coming_to_client to client_side/incoming_responses. Package names, workspace members, test_web_client dependencies, app.rs imports, and how_to_get_full_context.md updated to match.
