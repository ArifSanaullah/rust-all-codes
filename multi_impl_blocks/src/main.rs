#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width >  other.width
    }

    fn area(&self) -> u32 {
        self.height*self.width
    }
}
fn main() {
}
