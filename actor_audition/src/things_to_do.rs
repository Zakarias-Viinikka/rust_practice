use tokio::sync::oneshot;
use tokio::time::{Duration, sleep};

use crate::doers::PriorityLevel;

#[derive(Debug)]
pub enum ThingToDo {
    Thing1,
    Thing2,
    Thing3,
}

pub struct Instructions {
    pub thing_to_do: ThingToDo,
    pub priority_level: PriorityLevel,
    pub tx: oneshot::Sender<String>,
}

pub async fn do_thing(instructions: Instructions, delay: Duration) {
    match instructions.thing_to_do {
        ThingToDo::Thing1 => do_thing_1(instructions.tx, delay).await,
        ThingToDo::Thing2 => do_thing_2(instructions.tx, delay).await,
        ThingToDo::Thing3 => do_thing_3(instructions.tx, delay).await,
    }
}

pub async fn do_thing_1(tx: oneshot::Sender<String>, delay: Duration) {
    sleep(delay).await;
    tx.send("thing 1".to_string()).unwrap();
}

pub async fn do_thing_2(tx: oneshot::Sender<String>, delay: Duration) {
    sleep(delay).await;
    tx.send("thing 2".to_string()).unwrap();
}

pub async fn do_thing_3(tx: oneshot::Sender<String>, delay: Duration) {
    sleep(delay).await;
    tx.send("thing 3".to_string()).unwrap();
}
