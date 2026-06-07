#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]
#[path = "../boss_of_yrs.rs"]
mod boss_of_yrs;
use boss_of_yrs::BossOfYrs;
use serde_json::json;
use yrs::StateVector;
use yrs::types::ToJson;
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{Doc, GetString, ReadTxn, Text, Transact, Update};
/*
 * using this as a reference for figuring out this yrs stuff
 * https://docs.rs/yrs/latest/yrs/#quick-start
 */

fn main() {
    let boss_of_yrs = BossOfYrs::new();
    let doc = &boss_of_yrs.doc;

    let remote_doc = BossOfYrs::new();

    let text = vec![
        boss_of_yrs.doc.get_or_insert_text("block1"),
        boss_of_yrs.doc.get_or_insert_text("block2"),
    ];

    {
        let mut txn = boss_of_yrs.doc.transact_mut();

        text[0].insert(&mut txn, 0, "hello");
        text[0].insert(&mut txn, 5, " world");

        text[1].insert(&mut txn, 0, "bye");
        text[1].insert(&mut txn, 5, " jupiter");

        /* */

        {
            let mut remote_txn = remote_doc.doc.transact_mut();
            //stores info about what changes the remote document has and turns into a format that is cheaper to transmit over the network for example.
            let state_vector = remote_txn.state_vector().encode_v1();
            let update = txn.encode_diff_v1(&StateVector::decode_v1(&state_vector).unwrap());
            remote_txn.apply_update(Update::decode_v1(update.as_slice()).unwrap());
        }

        {
            let map = remote_doc.doc.transact();
            let map = map.root_refs();

            println!(
                "{:?}",
                map.into_iter().map(|(k, v)| k).collect::<Vec<&str>>()
            );
            //println!("{}", text[0].get_string(&boss_of_yrs.doc.transact()));
        }
    }

    //i can't use transact on the remote txn in the same scope because the transact mut hostages it for some reason

    //println!("{}", text[0].get_string(&remote_doc.doc.transact()));
    //println!("{}", text[1].get_string(&remote_doc.doc.transact()));

    //println!("{}", text[0].get_string(&boss_of_yrs.doc.transact()));
    //println!("{}", text[1].get_string(&boss_of_yrs.doc.transact()));
    //println!("{}", serde_json::to_string_pretty(&map).unwrap());
    println!("test");
}
