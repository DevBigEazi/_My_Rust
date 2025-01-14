use rand::Rng;
use core::num;
use std::{cmp::Ordering, io};

fn main() {
    println!("Number Guessing Game");

    let secret_number = rand::thread_rng().gen_range(10..20);

    // println!("The secret number is {}", secret_number);

    // Loop is used here to make sure the program run for ever
    loop {
        println!("Guess a new number between 10 and 20");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error: Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // the underscore here is a catchall value
        };

        println!("Your guess is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("This number is too small"),
            Ordering::Greater => println!("This number is too big"),
            // To avoid breaking of the program when a user input a non-number, we use break statement.
            // This means the prgram will only stop when a user won.
            Ordering::Equal => {
                println!("You are a winner!");
                break;
            }
        }
    }
}
