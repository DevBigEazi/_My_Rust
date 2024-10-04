fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // const variable must be decleared with its type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 12;

    let y = y + 22;

    {
        let y = y * 2;
        println!("The value of y in inner scope is {y}")
    }

    println!("{y}");

    let space = "    ";
    let space = space.len();

    // let mut space = "    ";  //The error says we’re not allowed to mutate a variable’s type:
    // space = space.len();

    println!("{space}");
}
