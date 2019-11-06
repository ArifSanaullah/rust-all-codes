mod front_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}
fn server_order() {}
mod back_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::server_order();
    }
    fn cook_order() {}
}
pub use self::back_house::Appetizer;
use crate::front_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    let soup = Appetizer::Soup;
    crate::front_house::hosting::add_to_waitlist();
    front_house::hosting::add_to_waitlist();
    let mut meal = back_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd Like to have {} toast please.", meal.toast);
    // meal.seasonal_fruit = String::from("Mango");

    let order_1 = back_house::Appetizer::Soup;
    let order_2 = back_house::Appetizer::Salad;
}
