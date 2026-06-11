mod folder_in_bin;

/* ...bin::*; works if the lib.rs has both:
 * pub mod folder_outside_of_bin;
 * and
 * pub use dog::*;
 */
use cargo_project_structure_test::folder_outside_of_bin::dog::*;

//i was confusing myself with the file names. file_only_bin_rs_needs is basically the equiavlent of ::dog:: in the above 'use' line.
use folder_in_bin::file_only_bin_rs_needs::*;

fn main() {
    let cat = Cat::new();
    let dog = Dog::new();

    println!("{}", cat.opinion);
    println!("{}", dog.legs);
}
