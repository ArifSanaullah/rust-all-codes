#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
fn main() {
    impl Rectangle {
        // add code here
        fn can_hold (&self, other: &Rectangle) -> bool {
            self.height > other.height && self.width > other.width
        }
    }
    let rect_1 = Rectangle {
        height: 100,
        width: 90,
    };
    let rect_2 = Rectangle {
        height: 90,
        width: 80,
    };
    let rect_3 = Rectangle {
        height: 80,
        width: 70,
    };
    let result_1 = rect_1.can_hold(&rect_2);
    let result_2 = rect_2.can_hold(&rect_3);
    let result_3 = rect_3.can_hold(&rect_1);
    println!("rect_1 can hold rect_2: {}\nrect_2 can hold rect_3: {}\nrect_3 can hold rect_1 {}",result_1,result_2,result_3);
}
