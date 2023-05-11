fn main() {
    // * Use a mutable integer variable
    let mut counter = 5;
    // * Use a while statement
    while counter >= 1 {
        // * Print the variable within the while loop
        println!("{:?}", counter);
        counter = counter - 1;
    }
    println!("done!")
}
