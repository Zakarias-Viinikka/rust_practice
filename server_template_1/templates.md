# template_0 vs template_1

template_0 is a frozen snapshot of the server_talker rewrite. It contains:

- server_talker — the rx/tx inbetween man that talks to the socket
- mock_server — dumb echo server
- error_stuff — error types
- web_client — the client that talks through server_talker

template_1 is a clone of template_0 and is where new work happens. template_0 is kept as a reference; work stays in template_1.

template_1 exists to combine in:

- the z_db protocol and error layer from server_ffi_recv
- the real server body and full Request set from web_socket_plus_android_ffi

The first step is the protocol crate. Right now server_talker speaks "id|payload" and the two donor projects speak JSON with message_id, Request, and Base64Bytes. Until that wire format is settled, the server side can't plug into server_talker.
