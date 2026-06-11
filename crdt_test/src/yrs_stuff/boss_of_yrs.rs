#![allow(dead_code)]
#![allow(unused_variables)]

use crate::yrs_stuff::yrs_params::*;

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

pub struct BossOfYrs {
    doc: Doc,
}

impl BossOfYrs {
    pub fn new() -> Self {
        Self { doc: Doc::new() }
    }

    pub fn insert_new_block(&self, block_content: String, block_meta_data: String) {
        /*
        let textRefExampleData1 = XmlTextPrelim::new("mock data");
        let textRefExampleData2 = XmlTextPrelim::new("rock sata");
        let metaDataExample1 = XmlTextPrelim::new("type_of_content: title,".to_string());
        let metaDataExample2 = XmlTextPrelim::new("type_of_content: regular text,");
        */
        let block_content = XmlTextPrelim::new(block_content);

        //https://docs.rs/yrs/latest/yrs/types/xml/struct.XmlElementRef.html
        /*
        pub struct XmlElementPrelim {
            pub tag: Arc<str>,
            pub attributes: HashMap<Arc<str>, String>,
            pub children: Vec<XmlIn>,
        }
         */
        //xml element has the feature i want, but the way it wants to store data is just weird for my purposes but i don't think there's an easier alternative? it wants an iter so i'll just pass in an empty block.
        let block_meta_data = XmlElementPrelim::new(block_meta_data, []);

        //XmlTextPrelim::new("a textref mock data");
        //text_blocks is the id for the 'struct that holds all of the blocks' or something like that.
        let mut txn = self.doc.transact_mut();

        /*
        i think yrs requires me to store the "text data" in a different  xml type than the metadat if i want them to be synced in different ways. and then the way i sync them is by keeping them in different maps and the way i know what metadata belongs to what block is by using the xmlMap key value.
        */
        let data_blocks = self.doc.get_or_insert_map("text_blocks");
        data_blocks.push_back(&mut txn, block_content);

        let data_blocks_metadata = self.doc.get_or_insert_map("text_blocks_metadata");
        data_blocks_metadata.push_back(&mut txn, block_meta_data);

        //
        // // this is how far i got ^^^
        // // idk if it works. but the rust analyzer isn't giving me errors. im tired so im gonna stop for now.
        //
        /*
        block.insert(&mut txn, type_of_block, TextPrelim::new(block_content));*/

        /*
        let mut txn = doc.transact_mut();

        let block = blocks.push_back(&mut txn, MapPrelim::default());
        block.insert(&mut txn, "type", "paragraph");
        let text = block.insert(&mut txn, "content", TextPrelim::new("Hello world"));
        */
    }
}

fn println_everything_doc_holds() {}
