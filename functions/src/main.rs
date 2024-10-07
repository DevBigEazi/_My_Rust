fn main() {
    println!("Hello, world!");

    anothere_function();

    new_function(10, 'A');
}

fn anothere_function() {
    println!("another function");
}

fn new_function(y: u8, z: char) {
    println!("The value of y and z is {y} and {z} respectively");
}
