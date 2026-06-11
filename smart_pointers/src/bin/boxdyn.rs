#![allow(warnings)]
use crate::LuckModifier::KidLuck;

use std::fmt::Debug;

#[warn(dead_code)]
fn main() {
    let mut person = Person {
        age: 5,
        name: Some("Will".to_string()),
        evil_bacteria_in_body: Vec::new(),
        luck_modifier: KidLuck,
    };

    add_bacteria_to_body(&mut person, RunnyNoseBacteria::new());
    add_bacteria_to_body(&mut person, RunnyNoseBacteria::new());
    add_bacteria_to_body(&mut person, RunnyNoseBacteria::new());
    add_bacteria_to_body(&mut person, FallAsleepForeverBacteria::new());
    add_bacteria_to_body(
        &mut person,
        BacteriaThatMakesYouEligibleToStayAtHomeInsteadOfGoingToWork::new(),
    );

    dbg!(person);
}

fn add_bacteria_to_body<T: EvilBacteria + 'static>(_person: &mut Person, new_bacteria: T) {
    let bacteria = RunnyNoseBacteria::new();
    _person.evil_bacteria_in_body.push(Box::new(new_bacteria));
}

//fn roll_the_bacteria_lottery() {}
#[derive(Debug)]
struct Person {
    age: u8,
    name: Option<String>,
    pub evil_bacteria_in_body: Vec<Box<dyn EvilBacteria>>,
    luck_modifier: LuckModifier,
}

trait EvilBacteria: Debug {
    fn new() -> Self
    where
        Self: Sized;
    fn get_infection_chance(&self) -> f32;
    fn set_infection_chance(&mut self, new_infection_chance: f32);
    fn mutate_bactera(&mut self) {
        //todo
    }
    fn multiply_singular_bacteria(multiplication_chance: &f32) -> bool
    where
        Self: Sized, /*A method without self has no way to be called through a trait object | apparently that's a thing. */
    {
        //todo
        true
    }
    fn survive_bacteria_threat_cycle(&self) -> Result<(), BacteriaKillCause>; //every bacteria should implement this differently. they have different things that can kill them.
    //fn attempt_bacteria_takeover(&self) -> Bool;//need to move this out. and be part of person not bacteria
}

/*
these are the fields a bacteria should have
name: String,
infection_chance: f32,
*/

macro_rules! bacteria_getter_n_setters {
    ($name:ident) => {
        fn get_infection_chance(&self) -> f32 {
            self.infection_chance
        }
        fn set_infection_chance(&mut self, new_infection_chance: f32) {
            self.infection_chance = new_infection_chance;
        }
    };
}
#[derive(Debug)]
struct RunnyNoseBacteria {
    infection_chance: f32,
}
impl EvilBacteria for RunnyNoseBacteria {
    bacteria_getter_n_setters!(RunnyNoseBacteria);
    fn survive_bacteria_threat_cycle(&self) -> Result<(), BacteriaKillCause> {
        /*todo */
        Ok(())
    }
    fn new() -> Self {
        Self {
            infection_chance: 0.2,
        }
    }
}

#[derive(Debug)]
struct FallAsleepForeverBacteria {
    infection_chance: f32,
}
impl EvilBacteria for FallAsleepForeverBacteria {
    bacteria_getter_n_setters!(FallAsleepForeverBacteria);
    fn survive_bacteria_threat_cycle(&self) -> Result<(), BacteriaKillCause> {
        /*todo */
        Ok(())
    }
    fn new() -> Self {
        Self {
            infection_chance: 0.3,
        }
    }
}

#[derive(Debug)]
struct BacteriaThatMakesYouEligibleToStayAtHomeInsteadOfGoingToWork {
    infection_chance: f32,
}
impl EvilBacteria for BacteriaThatMakesYouEligibleToStayAtHomeInsteadOfGoingToWork {
    bacteria_getter_n_setters!(BacteriaThatMakesYouEligibleToStayAtHomeInsteadOfGoingToWork);
    fn survive_bacteria_threat_cycle(&self) -> Result<(), BacteriaKillCause> {
        /*todo */
        Ok(())
    }
    fn new() -> Self {
        Self {
            infection_chance: 0.4,
        }
    }
}

#[derive(Debug)]
enum BacteriaKillCause {
    OldAge,
}
#[derive(Debug)]
enum LuckModifier {
    KidLuck,
    NormalPersonLuck,
    EverybodyElse,
}
