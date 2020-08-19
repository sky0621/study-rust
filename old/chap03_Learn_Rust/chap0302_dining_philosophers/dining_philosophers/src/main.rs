struct Philosopher {
    name: String,
}

// Philosopherk構造体に関する定義
impl Philosopher {
    // 関連関数（associated function）
    fn new(name: &str) -> Philosopher {
        Philosopher {
            // to_string()により、&str が指す文字列のコピーが作られる
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is donw eating.", self.name);
    }
}

fn main() {
    // 「Vec<T>」＝「可変長の配列」
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    for p in &philosophers {
        p.eat();
    }
}
