fn main() {
    let s = dangle();
    println!("{}", s);
    let s2 = &s;
    println!("{}", s2);
    println!("{}", s);
    println!("{}", *s2);
}

fn dangle() -> String {
    let s = String::from("abc");
    s
}
