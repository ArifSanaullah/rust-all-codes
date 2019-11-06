fn main() {
    let a = String::from(" Hello World");
    let b = first_word(&a);
    println!("First Word is: {}",b);
    let c = second_word(&a);
    println!("Second Word is: {}",c);
}
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item==b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item==b' ' {
            return &s[..];
        }
    }
    &s[..]
}