fn main() {
    // let y = 20;  // statement (statement does not return a value after processing)
    // let z = (let u = 25); //expression (expression returns a value after processing)
// let number = {
    // let x = 19;
    // x + 1
// }; // here number is statement and and two lines in number are expressions
// println!("{}",number);

let (value_1,value_2) = square(3,5.5); //arguments
println!("{} {}",value_1,value_2);

}
fn square(x: u32, y: f64) -> (u32,f64) { //parameters
    let result_1 = x*x; //statement
    let result_2 = y*y; //statement

    (result_1,result_2)
}
