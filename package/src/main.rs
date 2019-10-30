#![allow(dead_code)]
use package::details;
use package::front_of_house::hosting::add_to_wait_list;
use package::front_of_house::hosting::seat_at_table;
use package::front_of_house::serving::take_order as t_o;
fn main() {
    details();
    add_to_wait_list();
    seat_at_table();
    take_order();
    t_o();
}
fn take_order() {
    println!("function take_order called from binary crate (main)");
}
