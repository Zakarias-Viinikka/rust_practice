# server_ffi_recv

A websocket server plus a matching web client. The wire protocol shapes are
shared between server and client, and the DB layer comes from the external
z_db repo.

## Layout

- `server/` — the axum websocket server binary (`cargo run` target).
- `web_client/` — a Leptos CSR web client that talks to the server over websockets.
- `server_protocol/` — the shared shape of what goes over the wire:
  `Message`, `Response`, `Request`, `UnbuiltRequest`, `UnbuiltResponse`, and the
  helpers `build_msg`, `build_response`, `unbuild_msg`, `unbuild_response`.
- `error_stuff/` — `ServErr`, the workspace error enum. Wraps `DbError` from z_db
  and `ErrorDetails` for server-side errors with file/method info.

## Serialization split

- The shapes live in `server_protocol`.
- The serialization mechanics come from z_db's `protocol` crate via the `Convert`
  trait and the `Base64Bytes` wrapper.
- Callsites use `build_msg` / `build_response` to go out, and `unbuild_msg` /
  `unbuild_response` to come in. They never touch raw bytes.

## local_cargo.sh

A wrapper so cargo behaves a bit differently in this workspace.

#c r runs local_cargo.sh on my machine

- `c r` — runs the server (default workspace member is `server`).
- `c r web` — opens a new terminal, cd's into `web_client/`, runs `trunk serve`.


## ip.env

`web_client/src/ip.env` holds the server IP the client connects to. It is
gitignored (matches `**/*.env`), so a fresh clone won't have it. Create it
yourself before building the client:

    echo "127.0.0.1" > web_client/src/ip.env

Use whatever IP the server is reachable at if it's not localhost.
