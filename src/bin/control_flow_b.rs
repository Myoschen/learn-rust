fn main() {
    // * Use a variable set to any integer value
    let n = 7;
    // * Use an if..else if..else block to determine which message to display
    if n > 5 {
        // * Use the println macro to display messages to the terminal
        println!(">5");
    } else if n < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}
