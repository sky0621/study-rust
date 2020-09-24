fn main() {
    let r1 = even(2);
    let r2 = even(3);

    let i1 = r1.unwrap();
    println!("{:?}", i1);

    let i2 = r2.unwrap();
    println!("{:?}", i2);
}

fn even(n: i32) -> Option<i32> {
    match n % 2 {
        0 => Some(n),
        _ => None,
    }
}
