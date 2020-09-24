use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let filename = String::from("hello.txt");
    println!("{}", filename);

    let res = read_username_from_file(&filename);
    match res {
        Ok(str) => println!("str:{}", str),
        Err(e) => println!("Error"),
    };

    println!("{}", filename);

    let res2 = read_username_from_file2(&filename);
    match res2 {
        Ok(str) => println!("{:#?}", str),
        Err(e) => println!("{:#?}", e),
    };

    let filename3 = String::from("hello3.txt");
    match read_username_from_file3(&filename3) {
        Ok(_) => println!("Yes!"),
        Err(e) => println!("{:#?}", e),
    }

    let g = Guess::new(100);
    println!("{}", Guess::value(&g));
}

fn read_username_from_file(filename: &String) -> Result<String, io::Error> {
    let fl = File::open(filename);
    let mut fl = match fl {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match fl.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2(filename: &String) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3(filename: &String) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
