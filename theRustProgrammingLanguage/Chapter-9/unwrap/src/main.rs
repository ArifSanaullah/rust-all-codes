#![allow(unused_variables)]
use std::fs::File;
fn main() {
    // let f = File::open("hello.txt").unwrap();
    let g = File::open("hello.txt").expect("file not found.");
}