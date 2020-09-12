fn main() {
    part7();
}

fn part7() {
    let i = String::from("I");
    let my = String::from("my");
    let me = String::from("me");
    let s = format!("{}-{}-{}", i, my, me);
    println!("{}", s);
}

fn part6() {
    let mut d = "init_val".to_string();
    println!("{:?}", d);
    d.push_str(" add_val");
    println!("{:?}", d);

    let a = String::from("A");
    let b = String::from("B");
    let c = a + &b;
    // println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

fn part5() {
    let mut v = vec![1, 2, 3, 4, 5];
    let second = v.get(1);
    // v.push(6);

    println!("{:?}", v);
    println!("{}", second.unwrap());
}
fn part4() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("{}", third);

    let five = v.get(5);
    match five {
        Some(v) => println!("{}", v),
        None => println!("None!"),
    }
}

fn part3() {
    let x;
    {
        let y = &5;
        let m = Mee { x: y };
        x = m.x;
    }
    println!("{}", x);
}

struct Mee<'a> {
    x: &'a i32,
}

fn part2() {
    let y = &5;
    let b = Baa { x: *y };
    println!("{:?}", b);
    println!("{}", b.xx());
}

#[derive(Debug)]
struct Baa {
    x: i32,
}
impl Baa {
    fn xx(&self) -> i32 {
        self.x
    }
}

fn part1() {
    let y = &5;
    let f = Foo { x: *y };
    println!("{:?}", f);
}

#[derive(Debug)]
struct Foo {
    x: i32,
}
