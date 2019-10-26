fn main() {
    let rect = (100, 50);
    println!("The area of rectangle is {}.", area(rect));
}
fn area (dimensions : (u32,u32)) -> u32{
    dimensions.0*dimensions.1
}