use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

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

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    pub fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
    }
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let render = BufReader::new(f);
        run(render, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<T: BufRead>(render: T, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in render.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
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
