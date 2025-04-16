
mod library;

use crate::library::types::Types::{
    Member,
    Book,
    BookStatus,
    Library
};
use crate::library::traits::traits::{
    BookTrait,
    LibraryTrait
};

fn main() {
    // ==== Initialize a book =====//
    let book1 = Book::new(
        String::from("The Diary of a Wimpy Kid"),
        "1223-4432-4411".to_string(),
        "Jay B".to_string(),
        2005,
        BookStatus::Available,
    );
    
    // ===== Initialize an empty library ====== //
    let mut library: Library<Book, Member> = Library {
        books: Vec::new(),
        members: Vec::new()
    };
    
    // ===Add the book using the trait method ====//
    let success = library.add_book(book1);
    
    if success {
        println!("Book added successfully!");
    } else {
        println!("Failed to add book!");
    }
    
    // ===Check the number of books in the library ===//
    println!("Number of books in library: {}", library.books.len());
}