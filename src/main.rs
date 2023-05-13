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

use aggregator::{Summary, Tweet};

// This is the entry point of the application.
fn main() {
    // Display a message to the user.
    println!("hello rust");

    // learn module
    let a = something::A::new(1, 2);
    a.print();

    // learn trait
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 則新推文: {}", tweet.summarize());
}
