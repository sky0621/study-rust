fn main() {
    println!("Hello, world!");
    // let r = dangle();
    // println!("{}", r);

    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
    s.clear();
    println!("{}", s);

    let s2 = String::from("hello world");
    println!("{}-{}", &s2[0..4], &s2[7..])
}

// error[E0106]: missing lifetime specifier
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &String) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
