use crate::{
    doers::PriorityLevel,
    things_to_do::{Instructions, ThingToDo},
};

pub struct TelephoneManager {
    telephone_receiver: tokio::sync::mpsc::Receiver<Instructions>,
    normal_queue: Vec<ThingToDo>,
    priority_queue: Vec<ThingToDo>,
}

impl TelephoneManager {
    pub async fn create_telephone_manager(
        telephone_receiver: tokio::sync::mpsc::Receiver<Instructions>,
    ) {
        let mut telephone_manager = TelephoneManager {
            telephone_receiver,
            normal_queue: Vec::new(),
            priority_queue: Vec::new(),
        };

        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            loop {
                let wait_for_call = telephone_manager.telephone_receiver.recv().await;
                if let Some(instructions) = wait_for_call {
                    if instructions.priority_level == PriorityLevel::Priority {
                        telephone_manager
                            .priority_queue
                            .push(instructions.thing_to_do);
                    } else {
                        telephone_manager
                            .normal_queue
                            .push(instructions.thing_to_do);
                    }
                    telephone_manager.alert_queue_processor();
                }
            }
        });
    }

    fn process_queue(&mut self) {}
}
