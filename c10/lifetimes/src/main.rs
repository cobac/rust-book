/*
The result could be a reference to either argument
This lifetime annotation means that the result reference will be valid
as long as both argument references are valid
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Implementations always needs to be annotated if the struct holds refs
#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    // No &output, no annotation
    fn level(&self) -> i32 {
        3
    }
    // 3rd ref. elision rule: &output gets lifetime of &self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("{i:?}")
    }
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let _result = longest_with_an_announcement(string1.as_str(), string2, 5);
    }
}
