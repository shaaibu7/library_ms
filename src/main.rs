pub mod library;

use crate::library::types::Types::{
    Member,
    Book,
    BookStatus,
    MembershipType,
    Library
};
use crate::library::traits::Traits::{
    BookTrait,
     LibraryTrait
};




fn main() {
    println!("Hello, world!");

    let book1 = Book {
        title: String::from("Journey to the west"),
        isbn: "1223-4432-4455".to_string(),
        author: "Suleiman Shaaibu".to_string(),
        year: 2025,
        status: BookStatus::Available,
    };

    let mut library: Library<Book, Member> = Library {
        books: Vec::new(),
        members: Vec::new()
    };

    library.books.push(book1);
}
