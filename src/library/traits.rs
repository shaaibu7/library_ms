
use crate::library::types::Types::{
    Book,
    BookStatus,
    Member,
    Library
};

pub mod traits {
    use super::*;
    
    pub trait BookTrait {
        fn new(title: String, isbn: String, author: String, year: u64, status: BookStatus) -> Book;
    }


    pub trait LibraryTrait {
        fn add_book(&mut self, book: Book) -> bool;
    }
}

use self::traits::LibraryTrait;

impl LibraryTrait for Library<Book, Member> {
    fn add_book(&mut self, book: Book) -> bool {
        
        if self.books.iter().any(|b| b.isbn == book.isbn) {
            return false;
        }
        self.books.push(book);
        true
    }
}