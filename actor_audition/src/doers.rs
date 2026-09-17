use tokio::sync::{mpsc, oneshot};

use crate::things_to_do::{Instructions, ThingToDo};

pub struct Doer1 {
    pub telephone: mpsc::Sender<Instructions>,
    pub priority_level: PriorityLevel,
}
pub struct Doer2 {
    pub telephone: mpsc::Sender<Instructions>,
    pub priority_level: PriorityLevel,
}
pub struct Doer3 {
    pub telephone: mpsc::Sender<Instructions>,
    pub priority_level: PriorityLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PriorityLevel {
    Normal,
    Important,
}

impl Doer1 {
    pub fn new(telephone: mpsc::Sender<Instructions>) -> Self {
        Self {
            telephone,
            priority_level: PriorityLevel::Normal,
        }
    }
}

impl Doer2 {
    pub fn new(telephone: mpsc::Sender<Instructions>) -> Self {
        Self {
            telephone,
            priority_level: PriorityLevel::Normal,
        }
    }
}

impl Doer3 {
    pub fn new(telephone: mpsc::Sender<Instructions>) -> Self {
        Self {
            telephone,
            priority_level: PriorityLevel::Important,
        }
    }
}

trait Doer {
    fn priority_level(&self) -> &PriorityLevel;
}

impl Doer for Doer1 {
    fn priority_level(&self) -> &PriorityLevel {
        &self.priority_level
    }
}

impl Doer for Doer2 {
    fn priority_level(&self) -> &PriorityLevel {
        &self.priority_level
    }
}

impl Doer for Doer3 {
    fn priority_level(&self) -> &PriorityLevel {
        &self.priority_level
    }
}

impl GetPriorityLevel for Doer1 {
    fn priority_level(&self) -> &PriorityLevel {
        &self.priority_level
    }
}

impl GetPriorityLevel for Doer2 {
    fn priority_level(&self) -> &PriorityLevel {
        &self.priority_level
    }
}

impl GetPriorityLevel for Doer3 {
    fn priority_level(&self) -> &PriorityLevel {
        &self.priority_level
    }
}

pub trait GetPriorityLevel {
    fn priority_level(&self) -> &PriorityLevel;
}
//---
//
//Make a phone call
//
//---
pub trait MakeAPhoneCall {
    async fn make_phone_call(&self) -> Result<String, String>;
}
impl MakeAPhoneCall for Doer1 {
    async fn make_phone_call(&self) -> Result<String, String> {
        let (tx, rx) = oneshot::channel();
        self.telephone
            .send(Instructions {
                thing_to_do: ThingToDo::Thing1,
                priority_level: self.priority_level.clone(),
                tx,
            })
            .await
            .unwrap();

        let result = rx.await;
        result.map_err(|e| e.to_string())
    }
}

impl MakeAPhoneCall for Doer2 {
    async fn make_phone_call(&self) -> Result<String, String> {
        let (tx, rx) = oneshot::channel();
        self.telephone
            .send(Instructions {
                thing_to_do: ThingToDo::Thing2,
                priority_level: self.priority_level.clone(),
                tx,
            })
            .await
            .unwrap();
        let result = rx.await;
        result.map_err(|e| e.to_string())
    }
}

impl MakeAPhoneCall for Doer3 {
    async fn make_phone_call(&self) -> Result<String, String> {
        let (tx, rx) = oneshot::channel();
        self.telephone
            .send(Instructions {
                thing_to_do: ThingToDo::Thing3,
                priority_level: self.priority_level.clone(),
                tx,
            })
            .await
            .unwrap();
        let result = rx.await;
        result.map_err(|e| e.to_string())
    }
}
