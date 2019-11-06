fn main() {
    // string slice is static in size while String is non-static means it can be changed in size at any time in process.
    let s = String::new();
    println!("{}", s);

    let s1 = String::from("Hello from 'from method'");
    println!("{}", s1);

    // converting from str to String (converting a stacked string to heaped string)
    let s2 = "Hello world from str";
    let s2_to_str = s2.to_string();
    println!("{}", s2_to_str);

    // storing UTF-8 encoded text with String
    let urdu = String::from("ویڈیوز اور آڈیوز کے");
    println!("{}", urdu);

    // updating the string
    // push_str()
    let mut s3 = String::from("foo");
    let s4 = " bar";
    s3.push_str(s4);
    println!("{}", s3);

    // pushing a character in string
    let mut s5 = String::from("appl");
    // remember...! when pushing a character into a string then use single quotations.
    s5.push('e');
    println!("{}", s5);

    // concatenation with + operator
    let s6p1 = String::from("string six part one");
    let s6p2 = String::from("string six part two");
    // all the other parts are passed as refrence to concatenation. like following the second string is passed as &.
    let s6p3 = s6p1 + &s6p2;
    println!("{}", s6p3);

    // concatenation with format macro
    let s7p1 = String::from("string Seven Part One");
    let s7p2 = String::from("string Seven Part Two");
    let s7p3 = String::from("string Seven Part Three");
    let s7 = format!("{} - {} - {}", s7p1, s7p2, s7p3);
    println!("{}", s7);

    // indexing in string type is not possible
    // let s8 = String::from(PAkistan);
    // println!("{}",s8[1]);

    // how string store bytes
    println!(
        "{}, {}",
        (String::from("Pakistan")).len(),
        (String::from("Paquistão")).len()
    );

    // indexing in str
    let s9 = "Paquistão";
    let index_data = &s9[0..10];
    println!("{}", index_data);

    // iterating on String using chars()
    let s10 = "किस्तान";
    for c in s10.chars() {
        println!("{}", c);
    }


    // iterating on string using bytes()
    for c in "Pakistan".bytes() {
        println!("{}",c);
    }
}
