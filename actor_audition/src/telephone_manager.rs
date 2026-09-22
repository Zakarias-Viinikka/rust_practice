use crate::{queue_manager::Queues, things_to_do::Instructions};
use tokio::sync::mpsc;

pub struct TelephoneManager {
    telephone_rx: mpsc::Receiver<Instructions>,
}

impl TelephoneManager {
    pub fn create_telephone_manager(
        telephone_rx: mpsc::Receiver<Instructions>,
        queues: Queues,
        wake_tx: mpsc::Sender<()>,
    ) {
        let mut telephone_manager = TelephoneManager { telephone_rx };

        tokio::spawn(async move {
            loop {
                let Some(instructions) = telephone_manager.telephone_rx.recv().await else {
                    return;
                };

                if let Err(e) = queues.add_to_queue(instructions, &wake_tx) {
                    eprintln!("telephone_manager: {}", e);
                }
            }
        });
    }
}
