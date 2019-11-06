fn main() {
    // Q # 3​ . Write a Rust program to perform mathematical operations between two numbers.
    // Declare two float variables and assign some hard-coded values to them. Then print the result of
    // addition, subtraction, division, and multiplication in between these two variables
    let num_1: f64 = 34.4;
    let num_2: f64 = 24.7;
    let addition = num_1 + num_2;
    let subtract = num_1 - num_2;
    let divide = num_1 / num_2;
    let multiple = num_1 * num_2;
    println!("First number is {} \nSecond number is {} \nSum of two numbers is {} \nDifference of number one and number two is {}\nResult of dividing number one by number two is {}\nThe product of two numbers is {} ..",num_1,num_2,addition,subtract,divide,multiple);
}