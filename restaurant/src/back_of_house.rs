pub enum Appetizer {
    Soup,
    Salad,
}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Self {
        Self {
            toast: String::from(toast),
            seasonal_fruit: String::from("peach"),
        }
    }
}

fn fix_incorrect_order() {
    cook_order();
    // You can use "super" to go to the upper level mod of back_of_house
    super::deliver_order();
}

fn cook_order() {}
