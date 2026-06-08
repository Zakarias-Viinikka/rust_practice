#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

/*#[path = "../boss_of_yrs.rs"]
mod boss_of_yrs;
use boss_of_yrs::BossOfYrs;
#[path = "../params/yrs_params.rs"]
mod yrs_params;
use yrs_params::*;*/
mod yrs_stuff;
use crate::yrs_stuff::boss_of_yrs::*;

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
    let doc = &boss_of_yrs.doc;

    let data_blocks = doc.get_or_insert_array("text_blocks");

    {
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
    }
}
