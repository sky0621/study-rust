fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("{}", x);

    // Cannot assign twice to immutable variable
    // x = 4;

    // コピーセマンティクス
    let y = x;
    println!("{}", y);
    println!("{}", x);

    // ムーブセマンティクス
    let s = "Hello, Rust".to_string();
    println!("{}", s);
    let t = s;
    println!("{}", t);
    // println!("{}", s);

    let mut x2 = 123;
    println!("{}", x2);
    x2 = 456;
    println!("{}", x2);
}
