fn main() {
    println!("Hello, world!");
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{:?}", home);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", loopback);

    home.call();
    loopback.call();

    // Option -----------------------------
    let some_number = Some(5);
    println!("{:?}", some_number);
    let some_string = Some("a string");
    println!("{:?}", some_string);

    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);

    // match ------------------------------
    let none = plus_one(None);
    println!("{:?}", none);
    let six = plus_one(Some(5));
    println!("{:?}", six);

    // if let -----------------------------
    println!();
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(0u8) = some_u8_value {
        println!("eight!");
    }
}

enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("{}", i);
            Some(i + 1)
        }
    }
}
