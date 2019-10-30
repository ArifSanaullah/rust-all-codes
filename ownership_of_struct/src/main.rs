fn main() {
    #[derive(Debug)]
    struct Book {
        name: &str,
        author: &str,
        price: u16,
        availability: bool,
    }
    let book_a = Book {
        name: "Book A",
        author: "Author A",
        price: 234,
        availability: false,
    };

}
