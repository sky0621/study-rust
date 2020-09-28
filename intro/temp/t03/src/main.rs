fn main() {
    let y;
    let z;
    {
        let x = 5;
        y = &x;
        z = &x;
        dbg!(x);
        dbg!(y);
        dbg!(z);
    }
    // dbg!(y);
}
