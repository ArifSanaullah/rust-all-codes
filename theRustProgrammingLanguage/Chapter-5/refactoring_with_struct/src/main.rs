#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
fn main() {
    let rect_1 = Rectangle {
        height: 100,
        width: 50,
    };
    println!("Area of Rectangle is: {}", area(&rect_1));
    println!("{:#?}",rect_1);
}
fn area(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}
