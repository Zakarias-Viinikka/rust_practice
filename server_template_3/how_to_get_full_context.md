# How to get full context

Read these, and that's everything needed:

- server_template_3/plan.md: what this project is doing
- server_template_3/human_doc.md: what each crate does
- server_template_3/progress.md: what has been changed so far and what is still open
- the server_template_3 code itself. Layout:
  - shared/: error_stuff, server_protocol (used by both sides)
  - server_side/: server, server_db_schema, queue
  - client_side/: how_to_talk_to_server, incoming_responses, test_web_client

The queue shape was ported from actor_audition (its queue_manager.rs), and that port now lives in server_side/queue. You don't need actor_audition again unless you're comparing against the original.