fn main() {
    // Ownership
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    // Just integers, so they get pushed to the stack
    // the value `5` gets copied
    let x = 5;
    let y = x;
    println! {"{y}"};

    // Here the part of the stack of s1 is copied,
    // but the pointer from s2 points to the same heap memory as s1
    // We say that s1 was moved into s2
    let s1 = String::from("hello");
    let s2 = s1;
    println! {"{s2}"};
    // Not valid because "hello" is now owned by s2
    // println! {"{s1}"};

    // Clone does a deep copy and works
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // Functions
    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here
                            // println!("{s}"); fails

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
        println!("{x}");
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.
    {
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length_meh(s1);

        println!("The length of '{}' is {}.", s2, len);
    }

    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s = String::from("hello");
        println!("{s}");
        change(&mut s);
        println!("{s}");
    }

    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            println!("{r1}")
        } // r1 goes out of scope here, so we can make a new reference with no problems.
        let r2 = &mut s;
        println!("{r2}")
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        // let r3 = &mut s; // Fails
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point

        // Non-lexical lifetime (NLL)
        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    {
        println! {"{}", first_word("heey mate")};

        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
        assert_eq!(slice, [2, 3]);
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length_meh(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// With s: &String the function would only work with references to String
// With &str works with both literals and String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
