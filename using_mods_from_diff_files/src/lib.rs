#![allow(dead_code)]
mod front_of_house;
pub use crate::front_of_house::hosting;
fn eat_at_restaurent() {
    hosting::add_to_wailist();
}   