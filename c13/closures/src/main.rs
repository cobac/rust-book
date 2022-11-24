use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    _height: u32,
}

fn main() {
    {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }
    {
        // Borrowing
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }
    {
        // Borrows mutably
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);
        // Errors, mutable ref passed to borrows_mutably
        // println!("Before evaluating the closure: {:?}", list);
        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // Value of list moved into the closure
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
        // List is undefined now, this errors
        // println!("After defining closure: {:?}", list);
    }
    {
        let mut list = [
            Rectangle {
                width: 10,
                _height: 1,
            },
            Rectangle {
                width: 3,
                _height: 5,
            },
            Rectangle {
                width: 7,
                _height: 12,
            },
        ];

        {
            // .sort_by_key requires closures that implement FnMut
            // Works fine
            list.sort_by_key(|r| r.width);
            println!("{:#?}", list);
        }
        {
            /*
            Naively tries to count the no. of times the predicate of
            the sort function is called by pushing `value` into
            `sort_operations` multiple times
             */

            /*
            Doesn't work, because .sort_by_key only accepts FnMut
            closures. The closure only works once. Calling it multiple
            times fails because the closure captures `value` into the
            closure, and then transfers its ownership to
            sort_operations. Thus this closure only implements FnOnce.
             */

            /*

                let mut sort_operations = vec![];
                let value = String::from("by key called");
                list.sort_by_key(|r| {
                sort_operations.push(value);
                r.width
            });
                println!("{:#?}", list);
                println!("Predicate called {} times", sort_operations.len())
                     */
        }
        {
            /*
            In this case it works because the function can be
            safely called multiple times since it doesn't move
            values, although it mutates them and that's fine.
            */
            let mut value: i32 = 0;
            list.sort_by_key(|r| {
                value += 1;
                r.width
            });
            println!("{:#?}", list);
            println!("Predicate called {} times", value)
        }
        {
            // The predicate of .sort_by_key can mutate between comparisons!
            // ... which just breaks merge sort
            let mut list = [1, 2, 3, 4];
            let mut value: i32 = 0;
            list.sort_by_key(|r| {
                value += 1;
                (r * 1) < value
            });
            println!("{:#?}", list);
            println!("Predicate called {} times", value)
        }
    }
}
