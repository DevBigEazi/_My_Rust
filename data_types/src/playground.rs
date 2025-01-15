fn main () {
    let guess = "43";

    let guess: u32 = guess.parse().expect("Not a number");
    println!("Lets make a guess: {}", guess);
}