fn main() {
    println!("Hello, world!");
    let mut idx = 0;
    loop {
        idx = idx + 1;
        println!("loop count:{}", idx);
        if idx >= 100 {
            break;
        }
    }

    println!(" ");

    for x in 0..10 {
        println!("{}", x)
    }

    println!(" ");

    for (i, j) in (5..10).enumerate() {
        println!("{}-{}", i, j)
    }

    println!(" ");

    'outer: for x in 0..5 {
        'inner: for y in 0..5 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {} - y: {}", x, y);
        }
    }

    println!(" ");
}
