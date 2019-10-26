
// Q # 1. Write a rust program and define a user defined function that receives one argument of
// any suitable data type and print whether the number is positive, negative or equal to zero.
// (hint: if/else)

fn main() {
    number_sign(-0);
}
fn number_sign(x: i32) {
    if x < 0 {
        println!("Given number is negative");
    }else if x > 0 {
        println!("Given number is Postive");
    }else {
        println!("Given number is Zero");
    }
}
