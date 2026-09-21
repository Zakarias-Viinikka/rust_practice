# server_template_2 plan

server_template_1 stays as it is. server_template_2 is where the next work happens.

## What carries over

The contents of server_template_1, including its human_doc.md, and the shape of the actor_audition project in rust_practice.

## What changes

The server uses z_db instead of the mock.

One or more real requests get created, so the client can ask the server to do actual things rather than just Ping.

On the server side, receiving a request and doing a request get separated, so concurrency is possible. The queue shape comes from actor_audition.

## Notes

actor_audition has a bug: the queue needs to do the most recent item of the highest priority, but does the newest of the highest priority instead. Fix it here in the plan when the queue is ported, and fix it in actor_audition separately.
