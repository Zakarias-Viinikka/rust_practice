# server_template_3 plan

server_template_2 stays as it is. server_template_3 is where the next work happens.

## What carries over

All of server_template_2.

## What changes

The server stops doing requests directly. Instead, receiving a request and
doing it get separated, so concurrency is possible.

The queue shape comes from actor_audition: a priority list and a normal list,
oldest of the highest priority first, a wake channel so the worker doesn't
spin when the queue is empty, and an `add_to_queue` method that pushes and
wakes in one call.

## After that

Once the queue is in and working, refactor.
