fn main() {
    let a: u32 = 12;
    let b = &a;
    let c = &b;
    println!("{} {} {}",a,b,c);
    println!("and address of b is {:p}, and c is {:p}",b,c);
}
