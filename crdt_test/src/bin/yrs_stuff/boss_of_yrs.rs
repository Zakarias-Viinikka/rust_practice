#![allow(dead_code)]
#![allow(unused_variables)]

use crate::yrs_stuff::yrs_params::*;

use serde_json::json;
use std::sync::Arc;
use yrs::block::Item;
use yrs::types::ToJson;
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{
    Array, ArrayRef, Doc, GetString, MapPrelim, ReadTxn, Text, TextPrelim, Transact, Update,
};
use yrs::{StateVector, Transaction};

pub struct BossOfYrs {
    pub doc: Doc,
}

impl BossOfYrs {
    pub fn new() -> Self {
        Self { doc: Doc::new() }
    }

    fn insert_new_block(&self, p: InsertNewBlockParams) {
        let mut txn = doc.transact_mut();
        let block = data_blocks.push_back(&mut txn, MapPrelim::default());
        block.insert(&mut txn, type_of_block, TextPrelim::new(block_content));

        /*
        let mut txn = doc.transact_mut();

        let block = blocks.push_back(&mut txn, MapPrelim::default());
        block.insert(&mut txn, "type", "paragraph");
        let text = block.insert(&mut txn, "content", TextPrelim::new("Hello world"));
        */
    }
}
