mod lib;

fn main() {
    // let a = |num| num * num;
    // println!("{}", a(16));
    //
    // let x = 4;
    // let equal_to_x = |z| z == x;
    // let y = 4;
    // println!("{}", equal_to_x(y));

    let xx = vec![1, 2, 3];
    let eq_to_xx = move |z| z == xx;
    println!("{:?}", xx);
    let yy = vec![1, 2, 3];
    println!("{:?}", eq_to_xx(yy));
}
