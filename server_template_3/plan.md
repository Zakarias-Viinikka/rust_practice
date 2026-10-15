## plan is finished
# this is an old artifact

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

## these are my plan notes after i haven't worked on this project in a while and im preparing.

# things to do

- move code out of lib.rs lib.rs should just be for use x or pub mod stuff.

- update server schema to be more similar to client_table_blueprint schema

- need to write a doc that explains how the crate works, reintroducing context for myself is really hard, even though im somewhat familiar with how the code works.

- want to spliut up the crates and stuff so it's a workspace so that server stuff and client stuff aren't all mixed together too
