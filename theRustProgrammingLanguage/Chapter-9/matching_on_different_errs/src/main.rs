#![allow(unused_variables)]
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(error) => panic!("error creating file"),
            },
            _ => {
                panic!("you dont'have permits");
            }
        },
    };
}
