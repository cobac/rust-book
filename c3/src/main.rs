fn main() {
    //  Variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 2;
    println!("The value of x now is: {x}");

    {
        // Local scope
        let x = x * 2;
        println!("The value of x now is doubled: {x}");
    }

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // Redefinition fails
    // const THREE_HOURS_IN_SECONDS: u32 = 1;
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours.");

    // Possible to redefine with new type
    let spaces = "   ";
    let spaces = spaces.len();

    // Fails, different types
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // Data types

    println!("Spaces is: {spaces}");
    //let x: u32 = 5;
    // let y = x - 10; Compiler caches it, overflows

    let floored = 2 / 3; // => 0, round to nearest int
    println!("floored = {floored}");

    let power = i32::pow(2, 3);
    println!("power = {power}");

    let tup = (1, 2, 3);
    let (tup1, tup2, tup3) = tup;

    println!("Tuple values: {tup1}, {tup2}, {tup3}.");

    let tup0 = tup.0;
    println!("Rust uses 0-indexing, as tup0 is = {tup0}.");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Printing via debug, array does not implement a default fomater
    println!("a = {a:?}.");
}
