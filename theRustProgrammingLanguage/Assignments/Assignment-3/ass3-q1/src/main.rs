
// Q # 1. Write a Rust program​ , ​ “User Input” should be used, you can use any one of following ​ OR
// all of the following.
// ● User Input is must
// Function that receive or return arguments/parameters
// ● Control Flow if/else
// ● Control Flow Loop (loop, while, for)

use std::io;
fn main() {
    let ret_value = receives_and_returns_args(-35);
    if ret_value < 0{
        println!("Sum is Negative.");
    }else{
        println!("Sum is Positive.")
    }
    println!("The sum of the number passed in argument and in input is: {}",ret_value);
}
fn receives_and_returns_args(number: i32) -> i32 {
    println!("Please enter any whole number");
    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input).expect("Failed to read line.");
    let number_2: i32 = str_input.trim().parse().expect("Please enter Integers only.");
    println!("Number passed in argument is: {}",number);
    println!("The number given in input is: {}",number_2);
    let sum: i32 = number+number_2;
    sum
}