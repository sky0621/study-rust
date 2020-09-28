#[derive(Eq, PartialEq)]
struct APartialEq(i32);

#[derive(PartialEq, PartialOrd)]
struct BPartialOrd(f32);

fn main() {
    println!("{:?}", APartialEq(0) == APartialEq(1));
    println!("{:?}", BPartialOrd(1.0) > BPartialOrd(0.0));
}
