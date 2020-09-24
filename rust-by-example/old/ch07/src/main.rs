fn main() {
    println!("{}", part1());
    part2();
}

fn part1() -> i32 {
    3 + 5
}

fn part2() {
    let m: u32 = 5;

    let n = {
        let a = 2;
        let b = 3;
        a * b
    };

    println!("{}", m + n);
}
