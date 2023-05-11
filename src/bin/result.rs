// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
    // * The structure must contain the person's name and age
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        // * The Ok variant should contain the initialized structure, but only if the person is aged 21 or older
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_owned(),
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}

fn main() {
    // * Instantiate two `Adult` structure:
    //   * One should be aged under 21
    let child = Adult::new(15, "Anita");
    //   * One should be 21 or over
    let adult = Adult::new(21, "Sanjay");

    // * Use `match` to print out a message for each `Adult`:
    match child {
        //   * For the Ok variant, print any message you want
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        //   * For the Err variant, print out the error message
        Err(e) => println!("{e}"),
    }

    match adult {
        //   * For the Ok variant, print any message you want
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        //   * For the Err variant, print out the error message
        Err(e) => println!("{e}"),
    }
}
