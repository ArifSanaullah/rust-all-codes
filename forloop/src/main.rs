fn main() {
    for a in (0..10).rev() {
        println!("{}. hello world",a);
    }
    let lottery = [12,34,54,65,34];
    for number in lottery.iter() {
        println!("{}",number);
    }
}
