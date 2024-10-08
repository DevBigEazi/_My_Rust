fn main() {
    println!("Hello, world!");

    anothere_function();

    new_function(10, 'A');

    expression();

    let a = five();
    println!("The value of a is: {a}");

    let x = plus_one(20);
    println!("The value of a is: {x}");
}

fn anothere_function() {
    println!("another function");
}

fn new_function(y: u8, z: char) {
    println!("The value of y and z is {y} and {z} respectively");
}
/*Statements and Expressions
Function bodies are made up of a series of statements optionally ending in an
expression.Because Rust is an expression-based language, this is an important
distinction to understand.
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples. */

fn expression() {
    let y = {
        let x = 5;
        x + 2
    };

    println!("The value eof y is {y}");
}

fn five() -> i8 {
    5
}

fn plus_one(x: i8) -> i8 {
    x + 1
}
