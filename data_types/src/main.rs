fn main() {
    // two data type subsets: scalar and compound.
    /*  A scalar type represents a single value. Rust has four primary scalar types:
    integers, floating-point numbers, Booleans, and characters. You may recognize these
    from other programming languages. Let’s jump into how they work in Rust. */

    //These built-in integer types in Rust:
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Floating-Point Types
    /*  Rust also has two primitive types for floating-point numbers, which are numbers
    with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits
    and 64 bits in size, respectively. The default type is f64 because on modern CPUs,
    it’s roughly the same speed as f32 but is capable of more precision.
    All floating-point types are signed. */

    // Example;
    let a = 2.0;
    println!("{a}");

    let x: f32 = 2.4;
    println!("{x}");

    let y: f32 = -122.4;
    println!("{y}");

    // Numeric Operations
    /* Rust supports the basic mathematical operations you’d expect for all the number
    types: addition, subtraction, multiplication, division, and remainder. Integer
    division truncates toward zero to the nearest integer. The following code shows how
    you’d use each numeric operation in a let statement: */

    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("{quotient}");

    let truncated = -95 / 90; // Results in -1
    println!("{truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");

    // The Boolean Type
    /*As in most other programming languages, a Boolean type in Rust has two possible
    values: true and false. Booleans are one byte in size. The Boolean type in Rust is
    specified using bool. For example: */

    let t = true;
    println!("{t}");

    let f: bool = false; // with explicit type annotation
    println!("{f}");

    // Character types
    /*
    Rust’s char type is the language’s most primitive alphabetic type.
     */

    let char1 = '😂';
    println!("{char1}");

    let char2: char = 'z'; // with explicit type annotation
    println!("{char2}");

    let char3: char = ' '; // with explicit type annotation
    println!("{char3}");

    let char4: char = 'A'; // with explicit type annotation
    println!("{char4}");

    // Compound Types
    /*  Compound types can group multiple values into one type. Rust has two primitive
    compound types: tuples and arrays.*/

    // The Tuple Type
    /* A tuple is a general way of grouping together a number of values with a variety of
    types into one compound type. Tuples have a fixed length: once declared, they cannot
    grow or shrink in size.*/

    let tup: (i32, u8, f32) = (600, 255, 2.4);
    //we can use pattern matching to destructure a tuple value in order to get individual value
    let (x, y, z) = tup;
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of z is {z}");

    /* We can also access a tuple element directly by using a period (.) followed by the
    index of the value we want to access. As with most programming languages, the first index in a tuple is 0.
    For example: */
    let new_tup: (u8, f64, i16) = (200, 3.2, -30);
    let a = new_tup.0;
    let b = new_tup.1;
    let c = new_tup.2;
    println!("The value of 1st turple is {a}");
    println!("The value of 2nd turple is {b}");
    println!("The value of 3rd turple is {c}");

    // The Array Type
    /* Another way to have a collection of multiple values is with an array. Unlike a
    tuple, every element of an array must have the same type. Unlike arrays in some other
    languages, arrays in Rust have a fixed length. */

    let _num_arr: [i32; 5] = [1, 2, 3, 4, 5];

    /*Arrays are useful when you want your data allocated on the stack rather than the
    heap or when you want to ensure you always have a fixed number of elements. An array
    isn’t as flexible as the vector type, though. A vector is a similar collection type
    provided by the standard library that is allowed to grow or shrink in size. If you’re
    unsure whether to use an array or a vector, chances are you should use a vector.
    However, arrays are more useful when you know the number of elements will not need to
    change. For example, if you were using the names of the month in a program, you would
    probably use an array rather than a vector because you know it will always contain 12
    elements:
    */
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let first_month = months[0];
    let fifth_month = months[4];
    let tenth_month = months[9];

    println!("The 1st month in this array is {first_month}");
    println!("The 5th month in this array is {fifth_month}");
    println!("The 10th month in this array is {tenth_month}");

    /*You can also initialize an array to contain the same value for each element by
    specifying the initial value, followed by a semicolon, and then the length of the array
    in square brackets, as shown here: */
    let f = [3; 5]; // This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
    let access = f[0];
    
    println!("Access this array {access}");

    // let access2 = f[6];
    // println!("Access this array {access2}"); 
}
