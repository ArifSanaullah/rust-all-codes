fn main() {
    // let book_name_1 = String::from("Book_A");
    // let book_author_1 = String::from("Author_A");
    // let book_price_1 = 500;
    // let book_availablity_1 = true;
    // println!("{} {}", book_name_1,book_author_1);

#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    price: u16,
    availability: bool,
}
let book_1 = Book {
    name: String::from("Book A"),
    author: String::from("Autohr A"),
    price: 677,
    availability: true,
};
let book_2 = Book {
    name: String::from("Book B"),
    author: String::from("Autohr B"),
    price: 988,
    availability: false,
};
println!("{:#?}\n{:#?}",book_1,book_2);
}
