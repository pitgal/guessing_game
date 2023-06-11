use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries: u8 = 1;

    println!("Guess the number!");

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess:u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("Congratulations guess number is {}\n\
                         You win in {} tries!", guess, tries);
                break; 
            },
        }
        tries += 1;
    }
}