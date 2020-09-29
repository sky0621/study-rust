use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Taro",
    about = "sample"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let render = BufReader::new(f);

        for line in render.lines() {
            println!("{}", line.unwrap());
        }
    } else {
        println!("No file specified.");
    }
}

// fn builder_pattern() {
//     let matches = App::new("My RPN program")
//         .version("1.0.0")
//         .author("Taro")
//         .about("RPN sample")
//         .arg(
//             Arg::new("formula_file")
//                 .about("aaa")
//                 .value_name("FILE")
//                 .index(1)
//                 .required(false),
//         )
//         .arg(
//             Arg::new("verbose")
//                 .about("bbb")
//                 .short('v')
//                 .long("verbose")
//                 .required(false),
//         )
//         .get_matches();
//
//     match matches.value_of("formula_file") {
//         Some(file) => println!("File specified: {}", file),
//         None => println!("No file specified."),
//     }
//
//     let verbose = matches.is_present("verbose");
//     println!("Is verbosity specified?: {}", verbose);
// }
