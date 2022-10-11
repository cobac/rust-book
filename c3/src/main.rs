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

    // Functions
    some_print();
    print_num(3);
    print_labeled_measurement(5, 'h');

    let x = {
        let x = 2 + 2;
        x + 1
    };

    println!("Printing x: {x}");

    let x = five();
    println!("Printing x: {x}");

    // Control flow

    // ifs are pretty standard, they require bool type as the condition
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // Errors, have to have the same type
    // let number = if condition { 5 } else { "six" };

    println!("The value of number with condition \"{condition}\" is: {number}");

    // loop {
    //     println!("loooooop")
    // }

    // Can assign a return value to a loop after the break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("counter = {counter}");

        if counter == 10 {
            break counter * 2;
        }
    };

    // We can label loops to allow to break and continue specific loops
    println!("The result is {result}");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While is normal
    let mut countdown = 3;
    while countdown != 0 {
        println!("{countdown}!");

        countdown -= 1;
    }
    println!("LIFTOFF!!!");

    // For loops iterate over collections
    // I assume they have to implement some interface?
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn some_print() {
    println!("Printing something.");
}

// Need to annotate the type of arguments
fn print_num(x: i32) {
    println!("The number is {x}.");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Need to annotate the return type, unless it is ()
fn five() -> i32 {
    5
    // return 5;
    // 5; => error
}
