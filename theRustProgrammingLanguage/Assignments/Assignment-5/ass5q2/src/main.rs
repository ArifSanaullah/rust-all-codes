use std::io;
// Q # 2. Write a Rust program, ​ make a blank float type Vector, by using any type of loop
// get 10 input from user and save (push) in that float Vector, print that vector data and
// finally delete (pop) that vector data.

fn main() {
    let mut vec1: Vec<f64> = vec![];
    let mut i = 0;
    while i < 10 {
        let mut inp_one = String::new();
        io::stdin()
            .read_line(&mut inp_one)
            .expect("Failed to read line");
        let num: f64 = inp_one.trim().parse().expect("Expected a float number");
        i += 1;
        vec1.push(num);
    }
    println!("{:?}", vec1);
    let mut i = vec1.len();
    while i != 0 {
        vec1.pop();
        i -= 1;
    }
    println!("{:?}",vec1);
}
