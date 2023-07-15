fn main() {
    {
        let mut stack = Vec::new();

        // stack.push(1);
        // stack.push(2);
        // stack.push(3);
        for i in 1..=3 {
            stack.push(i)
        }

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    {
        let x = 'c';
        match x {
            'a'..='z' => print!("ey"),
            _ => print!("ye"),
        }
    }
    {
        let s = Some(String::from("Hello!"));

        // unused variables still bind / do ownership stuff
        // if let Some(_s) = s {
        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
    }
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }
    {
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }
    }
    {
        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}
