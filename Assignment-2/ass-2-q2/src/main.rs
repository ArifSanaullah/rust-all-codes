
// Q # 2. Write a rust program and define a user defined function that receives 4 arguments of
// different data types (integer, float, boolean, string) and print them on the console.

fn main() {
    let integer_num: u32 = 65;
    let float_num: f64 = 89.5;
    let bool_num: bool = true;
    let string_variable = String::from("I am string");


    four_elements(integer_num, float_num, bool_num, string_variable);
}
fn four_elements(integer_number: u32, float_number: f64, bool_number: bool, string_var: String) {
    println!("This is Integer {:#?}.\nThis is float {:#?}.\nThis is Boolean {:#?}.\nThis is String {:#?}.",integer_number,float_number,bool_number,string_var);
}