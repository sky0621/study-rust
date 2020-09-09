fn main() {
    let p1 = Player {
        id: 10001,
        name: String::from("Taro"),
    };
    println!("{:#?}", p1);
    println!("{:#?}", p1.hello());
}

#[derive(Debug)]
struct Player {
    id: i64,
    name: String,
}
impl Player {
    fn hello(&self) -> &String {
        &self.name
    }
}
