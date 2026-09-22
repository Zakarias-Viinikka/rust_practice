use crate::cli_gui::Stats;
use crate::doers::PriorityLevel;
use crate::queue_err::{ErrorDetails, QueueErr};
use crate::settings::Settings;
use crate::things_to_do::{Instructions, do_thing};
use crate::unwrap_recv;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::spawn;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Queues {
    pub priority_queue: Arc<Mutex<Vec<Instructions>>>,
    pub normal_queue: Arc<Mutex<Vec<Instructions>>>,
}

impl Queues {
    pub fn spawn_queue_manager(
        mut wake_rx: mpsc::Receiver<()>,
        stats: Stats,
        settings: Settings,
    ) -> Queues {
        let queues = Queues {
            priority_queue: Arc::new(Mutex::new(Vec::new())),
            normal_queue: Arc::new(Mutex::new(Vec::new())),
        };

        let queues_clone = queues.clone();

        spawn(async move {
            loop {
                let Some(()) = wake_rx.recv().await else {
                    return;
                };

                queues_clone.process_entire_queue(&stats, &settings).await;
            }
        });

        queues
    }

    pub async fn process_entire_queue(&self, stats: &Stats, settings: &Settings) {
        loop {
            let next = self.get_next_item_to_process();

            match next {
                Some(queue_item) => {
                    self.process_single_queue_item(queue_item, stats, settings)
                        .await;
                }
                None => break,
            }
        }
    }

    pub fn get_next_item_to_process(&self) -> Option<Instructions> {
        let next = {
            let mut priority_queue = self.priority_queue.lock().expect(LOCK_ERR);
            if priority_queue.is_empty() {
                None
            } else {
                Some(priority_queue.remove(0))
            }
        };
        let next = match next {
            Some(instr) => Some(instr),
            None => {
                let mut n = self.normal_queue.lock().expect(LOCK_ERR);
                if n.is_empty() {
                    None
                } else {
                    Some(n.remove(0))
                }
            }
        };
        next
    }

    pub async fn process_single_queue_item(
        &self,
        queue_item: Instructions,
        stats: &Stats,
        settings: &Settings,
    ) {
        //this is the actual doing part
        let response = do_thing(queue_item.thing_to_do.clone(), settings.delay()).await;

        // --- //
        //everything below is unrelated to actually doing the thing. it's for the the cli gui
        // --- //
        let name = format!("{:?}", queue_item.thing_to_do);
        let received_at = queue_item.received_at;
        let tx = queue_item.tx;
        let start = Instant::now();
        if let Err(response) = tx.send(response) {
            println!(
                "queue_manager: response receiver dropped (shutdown, or caller gave up before the response arrived): {}",
                response
            );
        }
        stats.record(name, received_at.elapsed(), start.elapsed());
    }

    pub fn add_to_queue(
        &self,
        instructions: Instructions,
        wake_tx: &mpsc::Sender<()>,
    ) -> Result<(), QueueErr> {
        match instructions.priority_level {
            PriorityLevel::Important => self
                .priority_queue
                .lock()
                .map_err(|e| {
                    QueueErr::LockErr(ErrorDetails {
                        err_msg: e.to_string(),
                        file: file!().to_string(),
                        method: "add_to_queue".to_string(),
                    })
                })?
                .push(instructions),
            PriorityLevel::Normal => self
                .normal_queue
                .lock()
                .map_err(|e| {
                    QueueErr::LockErr(ErrorDetails {
                        err_msg: e.to_string(),
                        file: file!().to_string(),
                        method: "add_to_queue".to_string(),
                    })
                })?
                .push(instructions),
        }
        unwrap_recv!(
            wake_tx.try_send(()),
            ErrorDetails {
                err_msg: "could not wake queue".to_string(),
                file: file!().to_string(),
                method: "add_to_queue".to_string(),
            }
        );
        Ok(())
    }
}

const LOCK_ERR: &str = "this should never happen: nothing panics while holding the queue lock";
