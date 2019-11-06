fn main() {
    let mut s = String::from("Pakistan");
    let mut result = length(&mut s);
    println!("{}",result);
    result.push_str("hello");
    println!("{}",result);
}

fn length(x: &mut String) -> String{
    x.push_str(" Zindabad");
    x.to_string()
}