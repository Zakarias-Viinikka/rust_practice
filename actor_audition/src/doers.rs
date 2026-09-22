use tokio::{
    spawn,
    sync::{mpsc, oneshot},
};

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

pub trait MakeAPhoneCall {
    fn make_phone_call(&self) -> impl std::future::Future<Output = ()> + Send;
}

impl MakeAPhoneCall for Doer1 {
    fn make_phone_call(&self) -> impl std::future::Future<Output = ()> + Send {
        let telephone = self.telephone.clone();
        let priority_level = self.priority_level.clone();
        async move {
            spawn(async move {
                let (tx, rx) = oneshot::channel();
                telephone
                    .send(Instructions {
                        thing_to_do: ThingToDo::Thing1,
                        priority_level,
                        received_at: std::time::Instant::now(),
                        tx,
                    })
                    .await
                    .unwrap();
                let _ = rx.await;
            });
        }
    }
}

impl MakeAPhoneCall for Doer2 {
    fn make_phone_call(&self) -> impl std::future::Future<Output = ()> + Send {
        let telephone = self.telephone.clone();
        let priority_level = self.priority_level.clone();
        async move {
            spawn(async move {
                let (tx, rx) = oneshot::channel();
                telephone
                    .send(Instructions {
                        thing_to_do: ThingToDo::Thing2,
                        priority_level,
                        received_at: std::time::Instant::now(),
                        tx,
                    })
                    .await
                    .unwrap();
                let _ = rx.await;
            });
        }
    }
}

impl MakeAPhoneCall for Doer3 {
    fn make_phone_call(&self) -> impl std::future::Future<Output = ()> + Send {
        let telephone = self.telephone.clone();
        let priority_level = self.priority_level.clone();
        async move {
            spawn(async move {
                let (tx, rx) = oneshot::channel();
                telephone
                    .send(Instructions {
                        thing_to_do: ThingToDo::Thing3,
                        priority_level,
                        received_at: std::time::Instant::now(),
                        tx,
                    })
                    .await
                    .unwrap();
                let _ = rx.await;
            });
        }
    }
}
