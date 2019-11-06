// Q # 1. Write a Rust program,
// (a) Make an enum ​ Direction ​ . ​ With variants forward, backward, left, right.
// (b) Make a user defined function with name ​ “check_direction” ​ . ​ That take a variable
// of Data Type ​ “Direction” ​ as parameter.
// (c) Inside the body of function ​ “check_direction” ​ . ​ Handle all cases with ​ “match”
// for example. If Direction::forward Then print “Vehicle is moved forward”.
// (d) Call ​ check_direction ​ with enum variant as an argument from ​ “main” ​ function.

fn main() {
    // println!("Hello, world!");
    let turn_right = Direction::Right;
    check_direction(turn_right);
}
#[derive(Debug)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}
fn check_direction(direction: Direction) {
    match direction {
        Direction::Backward => println!("Vehicle is moving backward."),
        Direction::Forward => println!("Vehicle is moving forward."),
        Direction::Left => println!("Vehicle is turning to left."),
        Direction::Right => println!("Vehicle is turning to right."),
    }
}