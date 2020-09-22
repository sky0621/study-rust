fn main() {
    let x: i32 = 3;
    borrow(&x);
    borrow(&x);
    borrow(&x);

    cp(x);
    cp(x);
    cp(x);

    let bx = Box::new(x);
    eat(bx);
    eat2(&bx);
}

fn borrow(b: &i32) {
    println!("{}", b);
}

fn cp(c: i32) {
    println!("{}", c);
}

fn eat(e: Box<i32>) {
    println!("{}", e);
}

fn eat2(e: &Box<i32>) {
    println!("{}", e);
}
