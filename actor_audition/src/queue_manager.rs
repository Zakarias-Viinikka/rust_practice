use crate::cli_gui::Stats;
use crate::settings::Settings;
use crate::things_to_do::{Instructions, do_thing};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::spawn;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct Queues {
    pub priority: Arc<Mutex<Vec<Instructions>>>,
    pub normal: Arc<Mutex<Vec<Instructions>>>,
}

impl Queues {
    pub fn create_queue_manager(
        mut wake_rx: mpsc::Receiver<()>,
        stats: Stats,
        settings: Settings,
    ) -> Queues {
        let queues = Queues {
            priority: Arc::new(Mutex::new(Vec::new())),
            normal: Arc::new(Mutex::new(Vec::new())),
        };

        let priority = Arc::clone(&queues.priority);
        let normal = Arc::clone(&queues.normal);

        spawn(async move {
            loop {
                let Some(()) = wake_rx.recv().await else {
                    return;
                };

                loop {
                    let next = priority.lock().unwrap().pop();
                    let next = match next {
                        Some(instr) => Some(instr),
                        None => normal.lock().unwrap().pop(),
                    };

                    match next {
                        Some(instr) => {
                            let name = format!("{:?}", instr.thing_to_do);
                            let start = Instant::now();
                            do_thing(instr, settings.delay()).await;
                            stats.record(name, start.elapsed());
                        }
                        None => break,
                    }
                }
            }
        });

        queues
    }
}
