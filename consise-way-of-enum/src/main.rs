fn main() {
    let ip4 = IPAddress::V4(String::from("127.0.0.5"));
    let ip6 = IPAddress::V6(127,2,26);
    println!("{:#?}, {:#?}",ip4,ip6);
}
#[derive(Debug)]
enum IPAddress {
    V4(String),
    V6(u32,u32,u32),
}