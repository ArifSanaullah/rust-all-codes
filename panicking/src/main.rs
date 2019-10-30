#![allow(unused_variables)]
use std::fs::File;
  //create //module //struct 
fn main() {
    let v = vec![1, 2, 36];
    // v[10];

    
    // Recoverable erros with Results
    let f = File::open("hello.txt");
    //            associated function
    let f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("File not found {:#?}",err);
        },
    };
}
