use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args);

    let mut f = File::open(cfg.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

fn prg01() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

fn prg02() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    // {}を探しています
    println!("Searching for {}", query);
    // {}というファイルの中
    println!("In file {}", filename);
}

fn prg03() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    // {}を探しています
    println!("Searching for {}", query);
    // {}というファイルの中
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
}
