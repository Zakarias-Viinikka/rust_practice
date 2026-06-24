use crate::people::HonestPerson;
use LiarInitialResponse::Answer;
use LiarInitialResponse::Silence;
use rand::Rng;

pub fn get_statement_from_honest_person(honest_person: &HonestPerson) -> KindOfStatement<'_> {
    KindOfStatement::FromHonest(&honest_person.statement)
}

pub fn try_to_talk_to_liar() -> KindOfStatement<'static> {
    if flip_a_coin() {
        KindOfStatement::FromLiar(Answer(invent_a_lie()))
    } else {
        KindOfStatement::FromLiar(Silence)
    }
}

pub enum KindOfStatement<'a> {
    FromHonest(&'a str),
    FromLiar(LiarInitialResponse),
}

pub enum LiarInitialResponse {
    Answer(&'static str),
    Silence,
}

//
// rand stuff below
//

fn flip_a_coin() -> bool {
    rand::random_bool(0.5)
}

fn invent_a_lie() -> &'static str {
    if one_in_three() {
        "water is green"
    } else if one_in_three() {
        "cats are great"
    } else {
        "dogs are evil"
    }
}

fn one_in_three() -> bool {
    rand::random_bool(1.0 / 3.0)
}
