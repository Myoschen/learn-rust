fn main() {
    // * Use a mutable integer variable
    let mut n = 1;
    // * Use a loop statement
    loop {
        // * Print the variable within the loop statement
        println!("{:?}", n);
        if n == 4 {
            // * Use break to exit the loop
            break;
        }
        n = n + 1;
    }
}
