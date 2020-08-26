use std::mem::take;

fn main() {
    println!("Hello, world!");

    let s = "aiueo";
    println!("{}", s);
    let mut s2 = "aiueo";
    println!("{}", s2);
    let s3 = s;
    println!("{}", s3);
    println!("{}", s);

    let t = String::from("aiueo");
    println!("{}", t);
    let mut t2 = String::from("aiueo");
    println!("{}", t2);
    let t3 = t;
    println!("{}", t3);
    // borrow of moved value: `t`
    // println!("{}", t);

    let p = String::from("ppp");
    takes_ownership(p);
    // borrow of moved value: `p`
    // println!("{}", p);

    let p2 = String::from("ppp2");
    let ret = takes_and_gives_back(p2);
    // borrow of moved value: `p2`
    // println!("{}", p2);
    println!("{}", ret);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
