#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

/*#[path = "../boss_of_yrs.rs"]
mod boss_of_yrs;
use boss_of_yrs::BossOfYrs;
#[path = "../params/yrs_params.rs"]
mod yrs_params;
use yrs_params::*;*/

use crdt_test::yrs_stuff::boss_of_yrs::*;

/*use serde_json::json;
use std::sync::Arc;
use yrs::block::Item;
use yrs::types::ToJson;
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{
    Array, ArrayRef, Doc, GetString, MapPrelim, ReadTxn, Text, TextPrelim, Transact, Update,
};
use yrs::{StateVector, Transaction};*/

fn main() {
    let boss_of_yrs = BossOfYrs::new();

    let example_data1 = "mock data".to_string();
    let example_meta_data1 = "type_of_content: title,".to_string();
    boss_of_yrs.insert_new_block(example_data1, example_meta_data1);
    /*{
        /*
        * InsertNewBlockParams<'a> {
            doc: &'a Doc,
            block_content: String,
            type_of_block: String,
            data_blocks: &'a ArrayRef,
        }
        */
        //let mock_data: Vec<Text> =

        let arrayLen = text_blocks.len(&txn);
        text_blocks.insert(&mut txn, arrayLen, block);
    }*/
}
