fn main() {
    part3();
}

fn part1() {
    let pi = 3.1415926535_f64;
    println!("{}", pi);

    let ipi = pi as u64;
    println!("{}", ipi);

    // [E0604] only `u8` can be cast as `char`, not `f64`
    // let cpi = pi as char;
    // println!("{}", cpi);

    // [E0605] non-primitive cast: `u64` as `std::string::String`
    // let cpi = ipi as String

    let int: u8 = 100;
    let cha = int as char;
    println!("{}", cha);
}

fn part2() {
    let a = 10u8;
    let b = -20i32;
    let c = 30u64;
    let d = 3.14f32;
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}

type IsLocal = bool;

fn part3() {
    let mut local: IsLocal = true;
    println!("{}", local);

    let f = false;
    local = f;
    println!("{}", local);
}
