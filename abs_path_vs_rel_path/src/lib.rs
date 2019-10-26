#![allow(dead_code)] //skips warning of unused functions and variables
pub fn details() {
    println!("The deadline of this course is August 25, 2020");
}
mod cust_exper {
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_wait_list() {
                println!("function add_to_waitlist called hosting module");
            }
        }
        pub mod serving {
            pub fn take_order() {
                println!("function take_order called from serving module");
            }
        }
    }
}
pub mod dinning {
    fn eat_at_restaurent() {
        // relative path
        super::cust_exper::front_of_house::serving::take_order();
        // absolute path
        crate::cust_exper::front_of_house::serving::take_order();
    }
}
