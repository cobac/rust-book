fn main() {
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

    println!("Spaces is: {spaces}");
}
