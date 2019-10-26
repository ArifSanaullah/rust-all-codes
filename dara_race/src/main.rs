fn main() {
    let mut a = String::from("Hello");
    {
    let b = &mut a;
    b.push_str("ppushed from b");
    println!("{}",b);
    }
    {
    let c = &mut a;
    c.push_str("string: &str from c");
    println!("{}",c);
    }
}
