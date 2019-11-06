fn main() {
    let a = 5;
    let b = a;
    println!("{}, {}",a,b);
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s2: {}, s1: {}",s2,s1);

}
