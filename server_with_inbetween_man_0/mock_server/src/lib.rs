use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

pub struct MockServer {
    pub outgoing_tx: Sender<String>,
    pub incoming_rx: Receiver<String>,
}

pub fn spawn() -> MockServer {
    let (outgoing_tx, outgoing_rx) = channel::<String>();
    let (incoming_tx, incoming_rx) = channel::<String>();

    thread::spawn(move || {
        while let Ok(msg) = outgoing_rx.recv() {
            println!("[mock_server] got: {}", msg);

            let reply = match msg.split_once('|') {
                Some((id, payload)) => format!("{}|mock reply to: {}", id, payload),
                None => format!("mock reply to: {}", msg),
            };

            println!("[mock_server] replying: {}", reply);
            if incoming_tx.send(reply).is_err() {
                break;
            }
        }
    });

    MockServer {
        outgoing_tx,
        incoming_rx,
    }
}
