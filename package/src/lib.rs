pub fn details() {
    println!("The deadline of this course is August 25, 2020");
}
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("function add_to_waitlist called hosting module");
        }
        pub fn seat_at_table() {
            println!("function seat_At_table called from hosting module");
        }
    }
    pub mod serving {
        pub fn take_order() {
            println!("function take_order called from serving module");
        }
        pub fn serve_order() {
            println!("function serve_order called from serving module");
        }
        pub fn take_payment() {
            println!("function take_payment called from serving module");
        }
    }
    fn eat_at_restaurent(){
    // crate::front_of_house::serving::take_order()
    crate::front_of_house::serving::take_order();
    // front_of_house::serving::serve_order();
}
}
