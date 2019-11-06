fn main() {
    #[derive(Debug)]
    struct Book {
        name: String,
        author: String,
        price: u16,
        availability: bool,
    }
    let mut book_1 = Book {
        name: String::from("Book_A"),
        author: String::from("Author_A"),
        price: 789,
        availability: true,
    };
    println!("Before Mutating : {}", book_1.author);
    book_1.author = String::from("Author_B");
    println!("After Mutating : {}", book_1.author);
}
