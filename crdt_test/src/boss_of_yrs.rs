#![allow(dead_code)]
#![allow(unused_variables)]

use yrs::*;

pub struct BossOfYrs {
    pub doc: Doc,
}

impl BossOfYrs {
    pub fn new() -> Self {
        Self { doc: Doc::new() }
    }
}
