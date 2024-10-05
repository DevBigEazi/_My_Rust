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
}
