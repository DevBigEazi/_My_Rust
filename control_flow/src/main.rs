/*An if expression allows you to branch your code depending on conditions. You provide a
condition and then state, “If this condition is met, run this block of code. If the
condition is not met, do not run this block of code.” */

fn main() {
    let number = 10;

    if number > 9 {
        println!("The condition is true!");
    } else {
        println!("The condition is false!");
    }

    /*Unlike languages such as Ruby and JavaScript, Rust will not automatically try to
    convert non-Boolean types to a Boolean. You must be explicit and always provide if
    with a Boolean as its condition. */

    // fn main() {
    //     let number = 3;

    //     if number { // error
    //         println!("number was three");
    //     }
    // }

    //If we want the if code block to run only when a number is not equal to 0,
    //for example, we can change the if expression to the following:

    let number = 8;
    if number != 2 {
        println!("The number was something other than two");
    }

    //for multiple conditions
    let number = 7;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
