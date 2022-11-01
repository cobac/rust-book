enum IpAddr {
    V4(String),
    V6(String),
}
enum IpAddrOther {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn print_other(n: u8) {
    println!("other was: {n}")
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Quarter(String),
    Whatever,
}

fn main() {
    let _home = IpAddr::V4(String::from("127.0.0.1"));

    let _loopback = IpAddr::V6(String::from("::1"));

    let _home_other = IpAddrOther::V4(127, 0, 0, 1);

    let _loopback_other = IpAddrOther::V6(String::from("::1"));

    // Option

    let x = 5;
    let y = Some(5);
    //let y: Option<i32> = None;

    let sum = x + y.expect("y can't be null");
    println!("{sum}");

    // Matching
    let dice_roll = 9;
    match dice_roll {
        3 => println!("3"),
        7 => println!("7"),
        // Catch all, allows for pseudo-non-exahustive patterns
        other => print_other(other),
        // _ => discard(),
        // nothing happens
        // _ => ()
    }

    // if let
    let coin = Coin::Whatever;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
