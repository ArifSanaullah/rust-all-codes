fn main() {

    let result = dangle();
    println!("{}",result);

    fn dangle() -> String {
        let s = String::from("Hello");
        s
    }
}
