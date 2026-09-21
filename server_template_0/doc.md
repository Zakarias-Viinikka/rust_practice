## web_client

Client talks to server_talker through rx tx. It calls "tell server to do this" with one tx,and receives "server responded with this" with a separate rx.


## server_talker

Three channels:

- one channel for receiveing send instructions
- one channel for sending response back
- one channel for state of connection: closed/error/connected

## server

dumb mock server. gets string. returns string back

## What's still missing

server and client don't have id's attached to requests or response nor any logic for storing unfulfilled requests
