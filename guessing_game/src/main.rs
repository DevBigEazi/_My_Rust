use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Number guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Pls input your guessing number!!");

    // println!("The secret number is {secret_number}");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Pls type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess is {}", guess);

        // Comparing the Guess to the Secret Number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// fn main() {
//     let x = 5;
//     let y = 10;

//     println!("x = {x} and y + 2 = {}", y + 2);
// }
