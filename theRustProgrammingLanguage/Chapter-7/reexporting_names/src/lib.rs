#![allow(dead_code)]
pub mod front_house {
    pub  mod hosting {
        pub fn add_to_wailist() {

        }
    }
}
pub use front_house::hosting;
pub fn eat_at_restaurent () {
    hosting::add_to_wailist();
}
use rand::Rng;
fn main() {
    let secret_no = rand::thread_rng().gen_range(1,101);
    println!("{}",secret_no);
}