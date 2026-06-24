#![allow(unused)]
mod people;
mod talk_to_people;

use people::*;
use talk_to_people::*;

fn main() {
    let honest_person = HonestPerson::new();
    let liar_person = LiarPerson::new();

    for _ in 0..5 {
        println!("the liars statement: {}", liar_person.get_statement());
        println!(
            "the liars truth: {} \n",
            get_truth_from_person(&liar_person)
        );
        println!("the honest statement: {}", honest_person.get_statement());
        println!(
            "the honest truth: {} \n",
            get_truth_from_person(&honest_person)
        );
    }
}

fn get_truth_from_person<T: Person>(person: &T) -> String {
    person.try_get_truth().into()
}

trait Person {
    fn try_get_truth(&self) -> Truth;
}

impl Person for HonestPerson {
    fn try_get_truth(&self) -> Truth {
        match get_statement_from_honest_person(&self) {
            KindOfStatement::FromHonest(statement) => {
                return Truth::HonestTruth(statement.to_string()); //statement;
            }
            _ => {
                unreachable!()
            }
        }
    }
}

impl Person for LiarPerson {
    fn try_get_truth(&self) -> Truth {
        match try_to_talk_to_liar() {
            KindOfStatement::FromLiar(LiarInitialResponse::Answer(answer)) => {
                return never_trust_a_liars_first_words(answer);
            }
            KindOfStatement::FromLiar(LiarInitialResponse::Silence) => {
                return Truth::LiarsTruth("The liars truth was to stay quiet".to_string());
            }
            _ => {
                unreachable!()
            }
        }
    }
}

fn never_trust_a_liars_first_words(the_lie: &str) -> Truth {
    Truth::LiarsTruth(format!(
        "the liar meant the opposite of the following: {}",
        the_lie
    ))
}

enum Truth {
    HonestTruth(String),
    LiarsTruth(String),
}

impl Into<String> for Truth {
    fn into(self) -> String {
        match self {
            Truth::HonestTruth(truth) => truth,
            Truth::LiarsTruth(truth) => truth,
        }
    }
}
