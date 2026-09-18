use crate::{doers::PriorityLevel, queue_manager::Queues, things_to_do::Instructions};
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

                match instructions.priority_level {
                    PriorityLevel::Important => queues.priority.lock().unwrap().push(instructions),
                    PriorityLevel::Normal => queues.normal.lock().unwrap().push(instructions),
                }

                wake_tx.try_send(()).ok();
            }
        });
    }
}
