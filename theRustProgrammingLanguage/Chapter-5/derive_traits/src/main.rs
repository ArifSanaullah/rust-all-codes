fn main() {
    #[derive(Debug)]
    struct Rectangle {
        height: u32,
        width: u32,
    }
    let rect_1 = Rectangle {
        height: 100,
        width:30,
    };
    println!("{:#?}",rect_1);
}
