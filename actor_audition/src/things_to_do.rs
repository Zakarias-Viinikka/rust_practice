use tokio::sync::oneshot;

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

fn do_thing(instructions: Instructions) {
    match instructions.thing_to_do {
        ThingToDo::Thing1 => {
            do_thing_1(instructions.tx);
        }
        ThingToDo::Thing2 => {
            do_thing_2(instructions.tx);
        }
        ThingToDo::Thing3 => {
            do_thing_3(instructions.tx);
        }
    }
}

pub fn do_thing_1(tx: oneshot::Sender<String>) {
    tx.send("thing 1".to_string()).unwrap();
}

pub fn do_thing_2(tx: oneshot::Sender<String>) {
    tx.send("thing 2".to_string()).unwrap();
}

pub fn do_thing_3(tx: oneshot::Sender<String>) {
    tx.send("thing 3".to_string()).unwrap();
}
