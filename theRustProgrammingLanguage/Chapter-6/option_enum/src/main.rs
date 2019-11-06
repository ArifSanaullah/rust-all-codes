fn main() {
    let number_1 = Route::Any(7);
    let some_string = Route::Any(String::from("Hello world"));
    let some_nano_value: Option<i32> = None;
    println!("{:#?}{:#?} {:#?}",number_1,some_string,some_nano_value);
}
#[derive(Debug)]
enum Route <T> {
    Any (T),
    None,
}