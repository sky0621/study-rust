use std::option::Option::Some;

fn main() {
    part10();
    // let sv = Some(11);
    // if let Some(x) = sv {
    //     println!("{}", x);
    // };
}

enum Message2 {
    Hello { id: i32 },
}

fn part10() {
    let msg = Message2::Hello { id: 9 };
    match msg {
        Message2::Hello { id: id_val @ 3..=7 } => {
            println!("found: {}", id_val);
        }
        Message2::Hello { id: 10..=12 } => {
            println!("other found");
        }
        Message2::Hello { id } => {
            println!("any");
        }
    }
}

fn part9() {
    let x = Some(4);
    let y = true;
    match x {
        Some(4) | Some(5) | Some(6) if y => println!("yes"),
        _ => println!("no"),
    }
}

fn part8() {
    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }
    println!("robot_name:{:?}", robot_name.unwrap());
}

fn part7() {
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("found: {}", name),
        None => (),
    }
    println!("robot_name:{:?}", robot_name);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn part6() {
    let origin = Point {
        x: 11,
        y: 22,
        z: 33,
    };
    match origin {
        Point { x, .. } => println!("{}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn part5() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("quit");
        }
        Message::Move { x, y } => {
            println!("x:{}, y:{}", x, y);
        }
        Message::Write(text) => println!("{}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Color red:{}, green:{}, blue:{}", r, g, b);
        }
    }
}

fn part4() {
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn part3() {
    let v = vec!['a', 'b', 'c'];
    for (idx, val) in v.iter().enumerate() {
        println!("{} is at index {}", val, idx);
    }
}

fn part2() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn part1() {
    let no = 100;
    match no {
        100 => println!("100"),
        _ => println!("Others"),
    };

    if let 100 = no {
        println!("100");
    }
}
