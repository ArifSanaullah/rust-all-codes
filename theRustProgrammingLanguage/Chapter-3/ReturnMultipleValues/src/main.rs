fn main() {
    let word = String::from("Pakistan");
    let (result,_result_1) =  length(word);
    println!("{},{}",_result_1,result);
}
fn length(x: String) -> (usize, String) {
    (x.len(),x)
}
