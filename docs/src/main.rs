//! # main
//! This contains the code that runs the program

pub mod name;

use docs::Greet::greeter;
use name::greeter as name_greeter;

fn main() {
    greeter();
    name_greeter(String::from("Benjamin"))
}
