// mod 指定要使用的模組
// Rust 會自動在以下位置尋找模組：
// - 該檔案
// - 該路徑下同名檔案
// - 該路徑下同名目錄內的 mod.rs (舊版風格，仍然支援的路徑形式)
mod back_of_house;
mod front_of_house;

// use 簡化路徑
pub use crate::front_of_house::hosting;

fn deliver_order() {}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("whole wheat");
    println!("I want {:?} bread, thanks", meal.toast);

    let order = back_of_house::Appetizer::Soup;

    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // with "use" to simplify the path
    hosting::add_to_waitlist();
}
