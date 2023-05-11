// * Use a struct for the grocery item
struct GroceryItem {
    // * Use two i32 fields for the quantity an id number
    quantity: i32,
    id: i32,
}

// * Create a function to display the quantity
fn display_quantity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}

// * Create a function to display the id number
fn display_id(item: &GroceryItem) {
    println!("id: {:?}", item.id);
}

fn main() {
    let my_item = GroceryItem {
        quantity: 3,
        id: 99,
    };
    display_quantity(&my_item);
    display_id(&my_item);
}
