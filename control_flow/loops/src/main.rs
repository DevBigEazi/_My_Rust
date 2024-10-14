fn main() {
    // Rust has three kinds of loops: loop, while, and for. Let’s try each one.

    /*The loop keyword tells Rust to execute a block of code over and over again forever
    or until you explicitly tell it to stop. */

    loop {
        // println!("Again and again!");

        // we can stop this manually or with break keyword wherever we want the prgram to stop.
        /* We also used continue in the guessing game, which in a loop tells the program
        to skip over any remaining code in this iteration of the loop and go to the next
        iteration.
        One of the uses of a loop is to retry an operation you know might fail, such as
        checking whether a thread has completed its job.
        You might also need to pass the result of that operation out of the loop to the
        rest of your code. To do this, you can add the value you want returned after the
        break expression you use to stop the loop; that value will be returned out of the
        loop so you can use it, as shown here:
        */

        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 100 {
                break counter * 2;
            }
        };

        println!("Here is the result: {result}");
    }
}
