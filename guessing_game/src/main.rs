use std::io;

fn main() {
    println!("Number guessing game!");
    println!("Pls input your guess");

    let mut _guess = String::new();

    io::stdin()
        .read_line(&mut _guess)
        .expect("failed to read line");

    println!("You guessed: {}", _guess);
}
