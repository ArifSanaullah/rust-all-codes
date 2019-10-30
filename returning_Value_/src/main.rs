fn main() {
    let result = gives_ownership();
    println!("{}", result);
    // let string = String::from("String is this");
    let result_1 = takes_and_gives_ownership(result);
}
fn gives_ownership() -> String {
    let s = String::from("Gives Ownership");
    s
}
fn takes_and_gives_ownership(x: String) -> String {
    x
}
