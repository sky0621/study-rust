use crate::Subject::Japanese;

fn main() {
    part3();
}

#[derive(Debug)]
enum Subject {
    Japanese,
    Math,
    English,
}

struct ExamResult {
    sub: Subject,
    score: u32,
}

fn grading(res: &ExamResult) {
    match res {
        &ExamResult {
            sub: Subject::Japanese,
            score,
        } if score > 85 => println!("{:?} OK", res.sub),
        &ExamResult {
            sub: Subject::Math,
            score,
        } if score > 90 => println!("{:?} OK", res.sub),
        &ExamResult {
            sub: Subject::English,
            score,
        } if score > 80 => println!("{:?} OK", res.sub),
        _ => println!("..."),
    }
}

fn part4() {
    let one = ExamResult {
        sub: Subject::Japanese,
        score: 80,
    };

    let two = ExamResult {
        sub: Subject::Math,
        score: 95,
    };

    let three = ExamResult {
        sub: Subject::English,
        score: 85,
    };

    grading(&one);
    grading(&two);
    grading(&three);
}

fn part3() {
    judge(100);
    judge(77);
    judge(0);
    judge(50);
}

fn judge(score: u32) {
    match score {
        0 => println!("Oh..."),
        7 | 77 => println!("Lucky!"),
        s @ 90..=100 => println!("Great! {}", s),
        _ => println!("..."),
    }
}

fn part2() {
    for n in 0..10 {
        if n % 4 == 0 {
            println!("OK:{}", n);
        }
    }

    println!();

    let pgs = vec!["Rust", "Golang", "Java", "Scala"];
    for pg in pgs.iter() {
        match pg {
            &"Rust" => println!("Yeah!"),
            _ => print!("{}, ", pg),
        }
    }
}

fn part1() {
    let mut n = 2;

    let res = loop {
        n += n * 5;
        if n > 1000 {
            break n;
        }
        println!("{}", n);
    };

    println!("res: {}", res);
}
