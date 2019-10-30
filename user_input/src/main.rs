use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read a line");
    println!("{}",s);
    let mut num: u32 = s.trim().parse().expect("Please enter numbers only.");
    num = num+1;
    println!("{}",num);
}
