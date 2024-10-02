use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Number guessing game!");

    let secret_number: String = rand::thread_rng().gen_range(1..=100).to_string();
    println!("The secret number is {secret_number}");

    println!("Pls input your guessing number!!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess is {}", guess);

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("Too small!"),
    //     Ordering::Greater => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }
}

// fn main() {
//     let x = 5;
//     let y = 10;

//     println!("x = {x} and y + 2 = {}", y + 2);
// }
