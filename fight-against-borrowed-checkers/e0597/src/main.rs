fn main() {
    let u = User::new("taro");
    println!("{:?}", u);
}

#[derive(Debug)]
struct User<'a> {
    name: &'a str,
}

impl<'a> User<'a> {
    fn new(name: &'a str) -> User<'a> {
        User { name }
    }
}
