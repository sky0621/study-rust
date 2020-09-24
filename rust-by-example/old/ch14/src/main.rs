#[derive(Debug)]
struct Empty;

#[derive(Debug)]
struct Null;

trait DoubleDrop<T> {
    fn double_drop(&self, _: &T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(&self, _: &T) {
        println!("double_drop");
    }
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(&null);
    println!("{:?}", empty);
    println!("{:?}", null);
}
