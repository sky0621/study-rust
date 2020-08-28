use book_se_ch10_trait::{NewsArticle, Summary, Tweet};

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
}
