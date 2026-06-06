fn main() {}

//fn roll_the_bacteria_lottery() {}

/*
 struct Person {
    age: u8,
    name: Option<String>,
    evil_bacteria_in_body: Vec<Box<dyn EvilBacteria>>,
    luckModifier: LuckModifier,
 }

trait EvilBacteria {
    fn mutate_bacteria(&mut self);
    fn multiply_bacteria(&multiplication_chance) -> Bool;
    fn survive_bacteria_threat_cycle(&self) -> Result<(), BacteriaKillCause>; //every bacteria should implement this differently. they have different things that can kill them.
    //fn attempt_bacteria_takeover(&self) -> Bool;//need to move this out. and be part of person not bacteria
}

/*
these are the fields a bacteria should have
name: String,
infection_chance: f32,
*/

macro_rules! bacteria_fields {
    () => {
        name: String,
        infection_chance: f32,
    };
}

macro_rules! impl_evil_bacteria {
    ($name:ident) => {
        impl EvilBacteria for $name {
            fn mutate_bacteria(&mut self) {
                //todo
                //should use self.infection_chance
            }
        }
    };
}

struct runny_nose_bacteria {bacteria_fields!();}
impl_evil_bacteria!(runny_nose_bacteria);
struct fall_asleep_forever_bacteria {bacteria_fields!();}
impl_evil_bacteria!(fall_asleep_forever_bacteria);
struct bacteria_that_makes_u_elligible_to_stay_at_home_instead_of_going_to_work {bacteria_fields!();}
impl_evil_bacteria!(bacteria_that_makes_u_elligible_to_stay_at_home_instead_of_going_to_work);

 enum LuckModifier {
    KidLuck,
    NormalPersonLuck,
    EverybodyElse
 }
 */
