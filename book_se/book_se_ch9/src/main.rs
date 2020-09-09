use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!(e);
            }
        },
        Err(err) => {
            panic!(err);
        }
    };
    sub();

    let f2 = File::open("hello3.txt").unwrap();
}

fn sub() {
    let f = File::create("hello2.txt");
    let f = match f {
        Ok(fl) => fl,
        Err(e) => {
            panic!("Tried to create file but there was a problem: {:?}", e);
        }
    };
}
