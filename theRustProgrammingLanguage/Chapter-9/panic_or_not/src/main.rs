#![allow(unused_variables)]
#![allow(dead_code)]
use std::net::IpAddr;
fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    // let x: u32 = "87".parse().unwrap();
    assert_eq!(home.is_ipv4(), true);
}
