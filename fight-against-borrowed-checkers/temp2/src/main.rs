use std::collections::HashMap;

fn main() {
    part2();
}

fn part2() {
    let mut kvs = HashMap::new();
    kvs.insert("k0".to_string(), "v".to_string());
    let mut abc = Abc { kvs };

    let mut n = 1;
    loop {
        part2b(&mut abc, n);
        if n > 2 {
            break;
        }
        n += 1;
    }
    println!("{:?}", abc);
}

#[derive(Debug)]
struct Abc {
    kvs: HashMap<String, String>,
}

fn part2b(abc: &mut Abc, n: i32) {
    println!("{:?}", abc);
    abc.kvs.insert(format!("k{}", n), "v".to_string());
}

fn part1() {
    let x = 0;
    let y = &x;
    let z = &y;
    let z2 = &x;

    println!("{}-{}-{}-{}", x, y, z, z2);
}
