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
    }
}

// This is the entry point of the application.
fn main() {
    // Display a message to the user.
    println!("hello rust");

    let a = something::A::new(1, 2);
    println!("{:?}", a);
}
