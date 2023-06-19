use std::io;
use rand::{self, Rng};
use std::cmp::Ordering;

fn main() {
    let secret_number:i32 = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Guess the number!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!"); 
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is less!"),
            Ordering::Greater => println!("Your guess is higher!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
//https://www.tokyoghoul.xyz