use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("bye");
    {
        let greeting_file_result = File::open("hello.txt");
        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            // Err(error) => panic!("Problem opening the file: {:?}", error),
            // If file doesn't exists, create it
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }
    {
        // Same as above, but with .unwrap_or_else and anonymous fn
        let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }
    {
        let _rslt = File::open("hello.txt");
        // rslt.unwrap() Unwraps Ok(), panics on Err
        // rslt.expect("Msg") Unwraps Ok(), panics on Err with custom Msg
    }
    {
        let usr = read_username_from_file();
        println!("{usr:?}");
    }
    {
        println!("{:?}", "test".lines().next().unwrap().chars());
        println!("{:?}", last_char_of_first_line(""));
        println!("{:?}", last_char_of_first_line("hey what"));
    }
    {
        let rslt = read_username_from_file();
        println!("Result => Option with .ok(): {:?}", rslt.ok());
        println!(
            "Option => Result with .ok_or(): {:?}",
            None::<i32>.ok_or(MyError)
        );
    }
    {}
}

fn read_username_from_file() -> Result<String, io::Error> {
    //    let mut username_file = File::open("hello.txt")?;
    //    // let username_file_result = File::open("hello.txt");
    //    // let mut username_file = match username_file_result {
    //    //     Ok(file) => file,
    //    //     Err(e) => return Err(e),
    //    // };

    //  let mut username = String::new();
    //  username_file.read_to_string(&mut username)?;
    //  Ok(username);
    //  // match username_file.read_to_string(&mut username) {
    //  //     Ok(_) => Ok(username),
    //  //     Err(e) => Err(e),
    //  // }

    // ... Or
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    fs::read_to_string("hello.txt")
}
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines() // iterator over lines
        .next()? // Some(First line) or returns None
        .chars() // iterator over characters
        .last()
}

#[derive(Debug)]
struct MyError;
