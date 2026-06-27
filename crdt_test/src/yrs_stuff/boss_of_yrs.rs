#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

use crate::yrs_stuff::boss_of_yrs;

use serde_json::json;
use std::sync::Arc;
use yrs::block::Item;
use yrs::types::ToJson;
use yrs::types::TypeRef::XmlText;
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{
    Any, Array, ArrayRef, Doc, GetString, Map, MapPrelim, ReadTxn, Text, TextPrelim, Transact,
    Update, XmlElementPrelim, XmlElementRef, XmlTextPrelim,
};
use yrs::{StateVector, Transaction};

use rand::prelude::*;

pub struct BossOfYrs {
    pub doc: Doc,
}

impl BossOfYrs {
    pub fn new() -> Self {
        Self { doc: Doc::new() }
    }

    pub fn insert_new_block(
        &self,
        block_content: String,
        block_meta_data: String,
        block_id: String,
    ) {
        /*
        let textRefExampleData1 = XmlTextPrelim::new("mock data");
        let textRefExampleData2 = XmlTextPrelim::new("rock sata");
        let metaDataExample1 = XmlTextPrelim::new("type_of_content: title,".to_string());
        let metaDataExample2 = XmlTextPrelim::new("type_of_content: regular text,");
        */
        {
            let block_content = XmlTextPrelim::new(block_content);
            //let block_meta_data = XmlElementPrelim::new(block_meta_data, []);

            // IMPORTANT
            // for some reason, if i put this code after
            // ...txn =...
            // then my code deadlocks.
            let data_blocks = self.doc.get_or_insert_map(doc_block_id()); //"text_blocks");

            let mut txn = self.doc.transact_mut();

            data_blocks.insert(&mut txn, block_id.clone(), block_content);
        }
        /*
        let data_blocks_metadata = self.doc.get_or_insert_map("text_blocks_metadata");
        data_blocks_metadata.insert(&mut txn, block_id, block_meta_data);
        */
    }

    pub fn show_doc_info(&self) {
        let map = self.doc.get_or_insert_map(doc_block_id());
        let mut txn = self.doc.transact();

        let json_representation = map.to_json(&txn);
        println!("{}", json_representation);
        /*if let Some(mapRef) = txn {
            println!("{}", mapRef.to_json(self.doc.transact()));
            //dbg!(data);
        }*/
    }
}

pub fn generate_key() -> String {
    let mut rng = rand::rng();
    let rnd_something: u64 = rng.random();
    dbg!(rnd_something);
    return rnd_something.to_string();

    //todo. just a temp gen for now
    /*
    i need to create a "generate new key method", but now i have the problem of 2 offline users working on the same page, having the small mathematical probability of generating the same key for 2 different blocks
    */
}

fn doc_block_id() -> String {
    "text_blocks".to_string()
}
