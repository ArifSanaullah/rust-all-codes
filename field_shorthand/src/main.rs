fn main() {
    fn build(name: String, author: String) -> Book {
        Book {
            name: name,
            price: 455,
            author: author,
            availability: true,
        }
    }
    let book_a = build(String::from("Book_A"), String::from("Author A"));
    println!("{:#?}", book_a);
}
#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    price: u16,
    availability: bool,
}
