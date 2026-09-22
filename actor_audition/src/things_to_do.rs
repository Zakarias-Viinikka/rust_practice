use tokio::sync::oneshot;
use tokio::time::{Duration, sleep};

use crate::doers::PriorityLevel;

#[derive(Debug, Clone)]
pub enum ThingToDo {
    Thing1,
    Thing2,
    Thing3,
}

pub struct Instructions {
    pub thing_to_do: ThingToDo,
    pub priority_level: PriorityLevel,
    pub received_at: std::time::Instant,
    pub tx: oneshot::Sender<String>,
}

pub async fn do_thing(thing_to_do: ThingToDo, delay: Duration) -> String {
    match thing_to_do {
        ThingToDo::Thing1 => do_thing_1(delay).await,
        ThingToDo::Thing2 => do_thing_2(delay).await,
        ThingToDo::Thing3 => do_thing_3(delay).await,
    }
}

pub async fn do_thing_1(delay: Duration) -> String {
    sleep(delay).await;
    "thing 1".to_string()
}

pub async fn do_thing_2(delay: Duration) -> String {
    sleep(delay).await;
    "thing 2".to_string()
}

pub async fn do_thing_3(delay: Duration) -> String {
    sleep(delay).await;
    "thing 3".to_string()
}
