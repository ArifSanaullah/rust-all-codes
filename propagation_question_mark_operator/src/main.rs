use std::fs::File;
use std::io::{self, Read};
fn main() {
    fn read_user_name_from_file()-> Result<String, io::Error> {
        let mut s = String::new();
        let f = File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s) //String type of s which is username in hello.txt
    }
    println!("{:#?}",read_user_name_from_file());
}
