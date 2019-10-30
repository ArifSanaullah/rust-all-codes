fn main() {
    let mut a = String::from("Hello");
    let b = &a;
    let c = &a;
    println!("{} {} {}",b,b,c);

    let d = &mut a;
    println!("{}",d);

}
