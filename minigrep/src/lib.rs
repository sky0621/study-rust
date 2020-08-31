use std::error::Error;
use std::fs::File;
use std::io::Read;

// Box<Error> ... トレイトオブジェクト(関数がErrorトレイトを実装する型を返すことを意味する)
pub fn run(cfg: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(cfg.filename).expect("file not found");

    let mut contents = String::new();
    // ?演算子は、現在の関数からエラー値を返す
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
