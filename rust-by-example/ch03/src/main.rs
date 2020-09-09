fn main() {
    part5();
}

fn part1() {
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

fn part2() {
    println!("{}", Signal::Red.mean());
    println!("{}", Signal::Yellow.mean());
    println!("{}", Signal::Green.mean());
}

enum Signal {
    Red,
    Yellow,
    Green,
}

impl Signal {
    fn mean(&self) -> &str {
        match self {
            Self::Red => "横断してはいけない",
            Self::Yellow => "横断をはじめてはいけない",
            Self::Green => "横断してもいい",
        }
    }
}

const PORT_OF_SERVER: i32 = 8080;

fn part3() {
    println!("{}", PORT_OF_SERVER);

    // invalid left-hand side of assignment E0070
    // PORT_OF_SERVER = 7070;
}

static LANG: &str = "Rust";

fn part4() {
    println!("{}", LANG);
}

fn part5() {
    let a = 1;
    println!("最初に a: {}", a);
    {
        // mut 無しでもシャドーイングにより束縛可能
        let a = 2;
        println!("ブロック内の a: {}", a);

        let b = 3;
        println!("ブロック内の b: {}", b);
    }
    println!("もう１回 a: {}", a);

    // cannot find value `b` in this scope E0425
    // println!("ブロック外の b: {}", b);
}
