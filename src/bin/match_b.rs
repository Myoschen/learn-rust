fn main() {
    // * Use a variable set to any integer
    let my_number = 2;

    // * Use a match expression to determine which message to display
    match my_number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        // * Use an underscore (_) to match an any value
        _ => println!("other"),
    }
}
