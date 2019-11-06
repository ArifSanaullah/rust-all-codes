#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    // add code here
    fn area (&self) -> u32 {
        self.width*self.height
    }
}
fn main() {
    // in function datatype of argument is explicitely defined but in method simply refrence of the argument is passed without datatype.
    let rect_1 = Rectangle {
        height:100,
        width: 50,
    };
    let rect_2 = Rectangle {
        height: 30,
        width: 60,
    };
    let result = rect_1.area();
    let result_2 = rect_2.area();

    println!("The area of recangle one  is: {}",result);
    println!("The area of recangle two is: {}",result_2);

}
