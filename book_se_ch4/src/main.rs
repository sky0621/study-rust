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
    println!("{}", ret);
    // borrow of moved value: `p2`
    // println!("{}", p2);

    let p3 = String::from("12345abcde");
    let us = calculate_length(&p3);
    println!("{}", p3);
    println!("{}", us);

    let mut ss = String::from("hello");
    change(&mut ss);
    println!("{}", ss);

    let ss1 = &mut ss;
    // error[E0499]: cannot borrow `ss` as mutable more than once at a time
    // let ss2 = &mut ss;
    println!("{}", ss1);
    // println!("{}", ss2);
    {
        let ss3 = &mut ss;
        println!("{}", ss3);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    // Cannot assign twice to immutable variable [E0384]
    // s = "baba";
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
