// * Use an enum for the tickets with data associated with each variant
enum Ticket {
    Standard(f64),
    Backstage(f64, String),
    Vip(f64, String),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(50.0, "Billy".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, "Amy".to_owned()),
    ];

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard ticket Price: {:?}", price),
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket Holder: {:?}, price: {:?}", holder, price)
            }
            Ticket::Vip(price, holder) => {
                println!("VIP ticket Holder: {:?}, price: {:?}", holder, price)
            }
        }
    }
}
