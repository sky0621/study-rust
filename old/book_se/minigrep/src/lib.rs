use std::env;
use std::env::Args;
use std::error::Error;
use std::fs::File;
use std::io::Read;

// Box<Error> ... トレイトオブジェクト(関数がErrorトレイトを実装する型を返すことを意味する)
pub fn run(cfg: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(cfg.filename).expect("file not found");

    let mut contents = String::new();
    // ?演算子は、現在の関数からエラー値を返す
    f.read_to_string(&mut contents)?;

    let results = if cfg.case_sensitive {
        search(&cfg.query, &contents)
    } else {
        search_case_insensitive(&cfg.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        // 1つ目はプログラム名なのでスキップ
        args.next();

        // 2つ目は検索文字列
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // 3つ目は検索対象ファイル名
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // Rustは
        // 安全で速く生産性も高い。
        // 3つ選んで。
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
