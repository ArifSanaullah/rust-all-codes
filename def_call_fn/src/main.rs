fn main() {
    let four = IPAddr::V4;
    let six = IPAddr::V6;
    route(four);
    route(six);
}

#[derive(Debug)]
enum IPAddr {
    V6,
    V4,
}
fn route(x: IPAddr) {

}