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

/*pub struct InsertNewBlockParams {
    block_content: String,
    type_of_block: String,
}*/
