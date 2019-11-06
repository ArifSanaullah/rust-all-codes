fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Write(String::from("Hello world"));
    let msg3 = Message::Move{x: 34, y: -53};
    let msg4 = Message::Change_Color(45,32,65);
    println!("{:#?}{:#?}{:#?}{:#?}",msg1,msg2,msg3,msg4);
}
#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    Move{x: i32, y:i32},
    Change_Color(u32, u32, u32),
}