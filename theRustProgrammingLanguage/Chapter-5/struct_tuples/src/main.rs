fn main() {
    #[derive(Debug)]
    struct Color (i32,i32,i32);
    // struct Points (i32,i32,i32);
    let black = Color(5,4,7);
    // let white = Points(5,6,36);
    another_function(black);
    // another_function(black);
fn another_function (x: Color){
    println!("{:#?}",x);
}
}
