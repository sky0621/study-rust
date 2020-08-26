fn main() {
    println!("Hello, world!");
    // let r = dangle();
    // println!("{}", r);
}

// error[E0106]: missing lifetime specifier
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &String) -> usize {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
