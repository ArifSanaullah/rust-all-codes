fn main() {
    let s = String::from("hello"); //s comes into scope
    takes_ownership(s); //s's ownershp is moved to takes_ownership function
    println!("{}",s); //s is out of scope and is dropped

    let num = 5; //num comes into scope
    makes_Copy(num); //num's scope is copied by makes_copy function
    println!("{}",num); //num is in the scope   

}
fn takes_ownership(x: String) { //x comes into scope
    println!("{}",x);
} //x is dropped
fn makes_Copy(x:i32) { //x comes into scope
    println!("{}",x);
}//x is dropped