#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]
#![allow(unused)]

use crdt_test::yrs_stuff::boss_of_yrs::{self, *};

fn main() {
    let boss_of_yrs = BossOfYrs::new();
    let key1 = generate_key();
    //let key2 = generate_key();

    let example_data1 = "mock data".to_string();
    let example_meta_data1 = "type_of_content: title,".to_string();
    boss_of_yrs.insert_new_block(example_data1, example_meta_data1, key1);

    boss_of_yrs.show_doc_info();
}
