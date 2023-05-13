// https://users.rust-lang.org/t/how-to-use-my-own-crate/2634
// access external crate
extern crate aggregator;

// This is a module
mod something {
    // You need to use "pub" keyword to expose content in the module
    // otherwise it is private.
    #[derive(Debug)]
    pub struct A {
        pub a: i32,
        b: i32,
    }

    impl A {
        pub fn new(a: i32, b: i32) -> Self {
            Self { a, b }
        }

        pub fn print(&self) {
            println!("a: {:?}, b: {:?}", self.a, self.b);
        }
    }
}

// [lifetime]
// 'a 會等同於 x 與 y 之間最小的生命週期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// [lifetime]
// 定義 ImportantExcerpt 擁有 part 參考，且宣告生命週期參數
// 表示 ImportantExcerpt 的實例不能比欄位 part 活得還久
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("please note: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use aggregator::{Summary, Tweet};

// This is the entry point of the application.
fn main() {
    // Display a message to the user.
    println!("hello rust");

    // [module]
    let a = something::A::new(1, 2);
    a.print();

    // [trait]
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 則新推文: {}", tweet.summarize());

    // [lifetime]
    let str1 = String::from("hello");
    let str2 = "rust";
    let result = longest(str1.as_str(), str2);
    println!("longest: {}", result);

    // [lifetime]
    let novel = String::from("Call me Ishmael, Some years ago...");
    let first_sentence = novel.split('.').next().expect("not fount '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let p = i.announce_and_return_part("announcement");
    println!("{}", p)
}
