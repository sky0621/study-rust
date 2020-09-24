use book_se_ch6::{plus_one, IpAddr};

mod lib;

fn main() {
    // part1()
    part2()
}

pub trait TReviewState {
    fn requestReview(&self);
}

pub struct UnRequested {
    pub id: i32,
}

impl TReviewState for UnRequested {
    fn requestReview(&self) {
        println!("UnRequested:{:?}", self.id);
    }
}

#[derive(Debug)]
pub enum ReviewState<TReviewState> {
    UnRequested: TReviewState,
    Waiting(i32),
    Pending(i32),
    Reject(i32),
    Approve(i32),
}
impl<TReviewState> ReviewState<TReviewState> {
    pub fn call(&self) {
        self.requestReview();
    }
}

fn part2() {
    let id = 100;
    let ur = ReviewState::UnRequested { id };
    println!("{:?}", ur);
}

fn part1() {
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
