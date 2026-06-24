#![allow(unused)]
mod people;
mod talk_to_people;

use people::*;
use talk_to_people::*;

use crate::talk_to_people::LiarInitialResponse::Silence;

fn main() {
    let honest_person = HonestPerson::new();
    let liar_person = LiarPerson::new();

    for _ in 0..5 {
        test_if_code_works(&honest_person, &liar_person);
    }
}

fn test_if_code_works(honest_person: &HonestPerson, lying_person: &LiarPerson) {
    println!("statement from honest person: ");
    if let KindOfStatement::FromHonest(statement) = get_statement_from_honest_person(&honest_person)
    {
        println!("{}", statement);
    } else {
        println!("couldn't get statement from honest person");
    }
    println!("");

    println!("statement from liar person: ");
    match try_to_talk_to_liar() {
        KindOfStatement::FromLiar(LiarInitialResponse::Answer(statement)) => {
            println!("{}", statement);
        }
        KindOfStatement::FromLiar(Silence) => {
            println!("Liar didn't respond");
        }
        _ => {
            println!("couldn't get statement from liar person (error in code logic)");
        }
    }
    println!("{}", "-".repeat(40));
}
