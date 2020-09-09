fn main() {
    // part1();
    part2();
    part3();
    part4();
    println!("{}", first_word("abcde"));
}

// fn part1() {
//     let r;
//     {
//         let x = 5;
//         // borrowed value does not live long enough
//         r = &x;
//     }
//     println!("r: {}", r);
// }

fn part2() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let res = longest(s1.as_str(), s2);
    println!("The longest string is {}", res);
}

// expected named lifetime parameter
// ライフタイム注釈
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn part3() {
    let s1 = String::from("abcd");
    {
        let s2 = String::from("xyz");
        let res = longest2(s1.as_str(), s2.as_str());
        println!("The longest string is {}", res);
    }
}

fn longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
// struct ImportantExcerpt {
//     // Missing lifetime specifier [E0106]
//     part: &str,
// }

fn part4() {
    let novel = String::from("Call me Ishmael. some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

fn first_word(s: &str) -> &str {
    s
}
