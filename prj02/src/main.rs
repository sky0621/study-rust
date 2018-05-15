fn main() {
    println!("println! is macro");

    let x = 5;
    println!("{}", x);

    let (a, b) = (1, 2);
    println!("{}+{}", a, b);

    let c: i32 = 32;
    println!("{}", c);

    // ERROR
    // let d = 5;
    // d = 10;

    let mut d = 5;
    d = 10;
    println!("{}", d);

    let e: i32 = 17;
    {
        let f: i32 = 18;
        println!("inner {}{}", e, f);
    }

    // ERROR
    // println!("outer {}{}", e, f);
    println!("outer {}", e);

    print_number(1234);

    let g = add_one(55);
    println!("{}", g);

    let f1: fn(x: i32) -> i32 = plus_one;
    println!("{}", f1(718));

    diverges();
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_number(x: i32) {
    println!("x is {}", x);
}

// 1つしか値を返せない
fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This function never returns!");
}