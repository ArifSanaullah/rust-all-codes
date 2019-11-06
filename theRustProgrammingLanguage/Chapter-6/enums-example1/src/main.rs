fn main() {
    let student1 = Student::Online;
    let student2 = Student::Onsite;
    println!("{:#?}{:#?}",student1,student2);
    let ip1 = IPAddrKind::Version4;
    let ip2 = IPAddrKind::Version6;
    println!("{:#?}{:#?}",ip1,ip2);
    let ip_1 = IP_address {
        kind: IPAddrKind::Version6,
        address: String::from("127.0.0.1"),
    };
    let ip_2 = IP_address {
        kind: IPAddrKind::Version4,
        address: String::from("127.0.0.2"),
    };
    println!("{:#?} {:#?}",ip_1,ip_2);
}
// example 1
#[derive(Debug)]
enum Student {
    Online,
    Onsite,
}
// example 2
#[derive(Debug)]
enum IPAddrKind {
    Version4,
    Version6,
}
// where is data
#[derive(Debug)]
struct IP_address {
    kind: IPAddrKind,
    address: String,
}

