use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));

        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    println!("count after b goes out of scope = {}", Rc::strong_count(&a));
}

fn part1() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{:?}", a);

    let b = Cons(3, Rc::clone(&a));
    println!("{:?}", b);

    let c = Cons(4, Rc::clone(&a));
    println!("{:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
