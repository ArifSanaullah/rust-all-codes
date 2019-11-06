#![allow(dead_code)]
pub mod front_house {
    #[derive(Debug)]
    pub struct Breafast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breafast {
        pub fn new(toast: String) -> Breafast {
            Breafast {
                toast,
                seasonal_fruit: String::from("Orange"),
            }
        }
    }
}
fn eat_at_restaurent() {
    let mut meal = front_house::Breafast::new(String::from("Wheat"));
    println!("{:#?}",meal);
    meal.toast = String::from("Barlay");
    println!("{:#?}", meal.toast);
}
