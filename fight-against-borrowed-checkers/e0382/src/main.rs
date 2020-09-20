fn main() {
    let name = String::from("taro");
    println!("{}", name);

    let u = User { name: &name };
    println!("{}", u.name);

    println!("{}", name);
}

struct User<'a> {
    name: &'a String,
}
