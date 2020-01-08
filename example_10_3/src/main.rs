// Trait

extern crate example_10_3;

use example_10_3::NewsArticle;
use example_10_3::Summary;

fn main() {
    println!("Hello, world!");

    let na = NewsArticle {
        headline : String::from("aaa"),
        location : String::from("bbb"),
        author : String::from("ccc"),
        content : String::from("ddd"),
    };

    println!("{}", na.summarize());
}
