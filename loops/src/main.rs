fn main() {
//     let mut counter = 0;
//     let value_counter = loop {
//         println!("hello world");
//         counter += 1;
//         if counter == 3 {
//             break counter
//         }
//     };
// println!("{}",value_counter);
    let mut counter = 0;
    while counter < 3 {
        println!("{}",counter);
        counter += 1;
    }
    let mut count = 0;
    let array = [12,34,23,45,323,54];
    while count < array.len() {
        println!("{}",array[count]);
        count += 1;
    }
}
