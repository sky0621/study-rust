extern crate minigrep;

use minigrep::Config;
use std::fs::File;
use std::io::Read;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// fn prg01() {
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);
// }
//
// fn prg02() {
//     let args: Vec<String> = env::args().collect();
//     let query = &args[1];
//     let filename = &args[2];
//     // {}を探しています
//     println!("Searching for {}", query);
//     // {}というファイルの中
//     println!("In file {}", filename);
// }
//
// fn prg03() {
//     let args: Vec<String> = env::args().collect();
//     let query = &args[1];
//     let filename = &args[2];
//     // {}を探しています
//     println!("Searching for {}", query);
//     // {}というファイルの中
//     println!("In file {}", filename);
//
//     let mut f = File::open(filename).expect("file not found");
//     let mut contents = String::new();
//     f.read_to_string(&mut contents)
//         .expect("something went wrong reading the file");
//     println!("With text:\n{}", contents);
// }
