use crate::library::types::Types::{Book, BookStatus};

pub mod traits {
    use std::io::StderrLock;

    use super::*;

    pub trait BookTrait {
        fn new(title: String, isbn: String, author: String, year: u64, status: BookStatus) -> Book;
    }

    pub trait LibraryTrait {
        fn add_book(&mut self, book: Book) -> bool;
        fn all_books(&self) -> Option<&Vec<Book>>;
    }
}
