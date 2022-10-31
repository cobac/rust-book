#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
#[derive(Debug)]
struct TupleStruct(i32, i32);

// Area program

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let _juan1 = build_user("test@test.test".to_string(), "Juan".to_string());
    let _juan2 = User {
        email: "test2@test.test".to_string(),
        .._juan1
    };

    println!("{_juan2:?}");

    let ts = TupleStruct(1, 2);
    println! {"{:?}", ts};
    println! {"{}", ts.0};
    println! {"{}", ts.1};

    // Area program

    let rect1 = Rectangle {
        // Inline debugging
        width: dbg!(30 + 1),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} units squared",
        rect1.area()
    );

    println!("{:?}", rect1);
    println!("{:#?}", rect1);

    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
