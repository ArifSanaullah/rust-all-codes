#![allow(dead_code)]
use rand::Rng;
use std::cmp::Ordering;
use std::io;
#[derive(Debug)]
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 || value > 101 {
            panic!(
                "You entered the number out of range. You entered {:#?}",
                value
            )
        }

        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    } //restricting us to not to initialize the value field given in main function.
}

fn main() {
    println!("Welcome To Guessing Game");

    let sec_num: i32 = rand::thread_rng().gen_range(1, 101);
    println!("secrete Number is {}", sec_num);
    loop {
        println!("Enter Your Guessed Number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let valid_num = Guess::new(guess);
        let guess = valid_num.value();
        match guess.cmp(&sec_num) {
            Ordering::Equal => {
                println!("Your Win...!");
                break;
            }
            Ordering::Greater => println!("You Guessed too high"),
            Ordering::Less => println!("You guessed too low."),
        }
    }
}
