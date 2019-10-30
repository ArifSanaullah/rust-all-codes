#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn square (size:u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}
fn main() {
    let result = Rectangle::square(40);
    println!("{:#?}",result);
}
