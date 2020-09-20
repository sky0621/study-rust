fn main() {
    let n1 = 1.234_f32;
    let n2: f64 = n1 as f64;
    println!("{}", n1);
    println!("{}", n2);

    let m1 = 2.3456789012345_f64;
    let m2: f32 = m1 as f32;
    println!("{}", m1);
    println!("{}", m2);
}
