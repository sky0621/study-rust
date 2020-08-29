use book_se_ch10_trait::{NewsArticle, Other, Summary, Tweet};
use std::fmt::{Debug, Display};

fn main() {
    let news1 = NewsArticle {
        headline: String::from("みだし"),
        location: String::from("ロケーション"),
        author: String::from("たろー"),
        content: String::from("コンテンツの中身です。"),
    };
    println!("{}", news1.summarize());

    let tw1 = Tweet {
        username: String::from("じろー"),
        content: String::from("じろーのコンテンツ。"),
        reply: true,
        retweet: true,
    };
    println!("{}", tw1.summarize());

    let oth = Other {
        other_name: String::from("その他"),
    };
    println!("{}", oth.summarize());

    some_function(32, "abc");
}

fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{}", t);
    println!("{:?}", u);
    100
}
