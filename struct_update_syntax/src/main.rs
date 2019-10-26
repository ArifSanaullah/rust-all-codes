fn main() {
    #[derive(Debug)]
    struct Book {
        name: String,
        author: String,
        price: u16,
        availability: bool,
    }
    let book_a = Book {
        name: String::from("Book A"),
        author: String::from("Author A"),
        price: 566,
        availability: true,
    };
    let book_b = Book {
        name: String::from("Book B"),
        author: String::from("Author B"),
        price: book_a.price,
        availability: book_a.availability,
    };
    let book_c = Book {
        name: String::from("Book C"),
        ..book_b
    };
    println!("{:#?}\n{:#?}",book_a,book_c);
}
