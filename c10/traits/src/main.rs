use core::fmt::{Debug, Display};

trait Summary {
    fn summarize_author(&self) -> String {
        String::from("Anonymous")
    }
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

#[allow(dead_code)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

struct DummyContent;
impl Summary for DummyContent {}

fn notify(item: &impl Summary) {
    // Syntax sugar for:
    // fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

// Equivalent expressions
fn _notify_both(_item1: &impl Summary, _item2: &impl Summary) {}
// Allows us to force both arguments to be of the same type
fn _notify_both_<T: Summary>(_item1: &T, _item2: &T) {}

fn _notify_displayable(_item: &(impl Summary + Display)) {}
fn _notify_displayable_<T: Summary + Display>(_item: &T) {}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
fn _some_function<T, U>(_t: &T, _u: &U) -> ()
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct _Pair<T> {
    x: T,
    y: T,
}

// For all Pairs
impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Conditionally for all Pairs(T), where T implements Display and PartialOrd
impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
    println!("tweet: {}", tweet.summarize());
    println!("dummy: {}", DummyContent.summarize());
}
