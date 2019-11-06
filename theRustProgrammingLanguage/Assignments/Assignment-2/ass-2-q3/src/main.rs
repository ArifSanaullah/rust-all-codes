// Q # 3. Write a rust program and define a user defined function that receives a number and
// return the number itself and its square by using tuple

fn main() {
    let (given_number, squared_number) = number_square(6);
    println!(
        "Given Number is: {}\nSquare of the number is:{}",
        given_number, squared_number
    );
}
fn number_square(x: i32) -> (i32, i32) {
    return (x, x * x);
}
