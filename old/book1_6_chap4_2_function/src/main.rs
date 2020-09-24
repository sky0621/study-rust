fn main() {
    println!("Hello, world!");
    println!("{}+{}={}", 3, 5, plus(3, 5));

    // let mut y = 5;
    // let x = (y = 6);
    // println!("{}",x);

    diverges();
}

fn plus(a: i32, b: i32) -> i32 {
    // Rustは式ベース
    a + b
}

// 発散する関数＝値を返さない関数
fn diverges() -> ! {
    panic!("panic!")
}
