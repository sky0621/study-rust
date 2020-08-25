fn main() {
    println!("Hello, world!");

    const MAX_POINTS: i32 = 100_000;
    println!("{}", MAX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("{}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("{}-{}-{}", a, b, c);

    let z = 5;
    let y = {
        let z = 3;
        z * 3
    };
    println!("{}", y);

    let condition = true;
    let number = if condition { 1 } else { 0 };
    println!("{}", number);

    let a = [10, 20, 30, 40, 50];
    for ele in a.iter() {
        println!("{}", ele);
    }
}
