use crate::library::types::Types::{Book, BookStatus};

pub mod traits {
    use super::*;

    pub trait BookTrait {
        fn new(title: String, isbn: String, author: String, year: u64, status: BookStatus) -> Book;
    }

    pub trait LibraryTrait {
        fn add_book(&mut self, book: Book) -> bool;
    }
}
