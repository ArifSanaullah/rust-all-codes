#![allow(dead_code)]
#![allow(unused_variables)]
mod front_house {
    #[derive(Debug)]
    pub enum Appetizer {
        Salad,
        Soup,
    }
    pub mod hosting {
        pub fn add_waitlist() {
            println!("Add to Waitlist from hosting module in front house");
        }
    }
    pub mod serving {
        pub fn take_order() {
            println!("take_order from serving module in front house");
        }
    }
}
use crate::front_house::serving;
use self::front_house::hosting;
pub fn eat_at_restaurent() {
    let meal1 = front_house::Appetizer::Salad;
    let meal2 = front_house::Appetizer::Soup;
    serving::take_order();
    hosting::add_waitlist();
}
