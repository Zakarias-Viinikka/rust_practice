#![allow(unused)]
#[derive(Debug)]
pub struct HonestPerson {
    pub statement: String,
}

impl HonestPerson {
    pub fn new() -> Self {
        let statement = "The weather is still weathering today".to_string();
        Self { statement }
    }
}

#[derive(Debug)]
pub struct LiarPerson {
    pub statement: String,
}

impl LiarPerson {
    pub fn new() -> Self {
        let statement = "I would never lie about anything".to_string();
        Self { statement }
    }
}
