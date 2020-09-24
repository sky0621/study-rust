fn main() {
    let upper = 1000;

    // 値の蓄積用
    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            // 奇数なら値の継ぎ足し
            acc += n_squared;
        }
    }
    println!("{}", acc);

    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n_squared| acc + n_squared);
    println!("{}", sum_of_squared_odd_numbers);
}

fn is_odd(n: u32) -> bool {
    n % 2 != 0
}
