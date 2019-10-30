// Q# 2. Write a Rust Program,
// (a) Make a user defined function with name ​ “add_one” ​ .
// 1) ​ add_one ​ function is required to plus one to its parameter.
// 2) take a variable of Data Type ​ Option<i32> ​ as parameter.
// 3) return a value of Data Type ​ Option<i32> . ​
// (b) Handle both cases for ​ Option<i32> ​ enum with ​ match o
// perator.
// (c) From main function pass ​ some v ​ ariant and a ​ none ​ variant to the
// function ​ “add_one” ​ .
// (d) Print the return value from ​ “add_one” ​ function in ​ main function ​ .

fn main() {
    println!("{:?}", add_one(Some(6)));
    println!("{:?}", add_one(None));
}
fn add_one(variable: Option<i32>) -> Option<i32> {
    match variable {
        None => None,
        Some(i) => Some(i + 1),
    }
}
