fn main() {
    let msg1 = Message::Write(String::from("Hello world"));
    msg1.call();
    let msg2 = Message::Quit;
    msg2.call();
    let msg3 = Message::Change_Color(45,65,24);
    msg3.call();
}
#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    Move{x: i32, y: i32},
    Change_Color(u32,u32,u32),
}
impl Message {
    fn call (&self) {
        println!("{:#?}",self);
    }    
}