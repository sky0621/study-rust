use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

// recursive type `List` has infinite size E0072
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
