use crate::library::types::Types::{Book, BookStatus};

pub mod traits {
    use crate::library::types::Types::Member;

    use super::*;

    pub trait BookTrait {
        fn new(title: String, isbn: String, author: String, year: u64, status: BookStatus) -> Book;
    }

    pub trait LibraryTrait {
        fn add_book(&mut self, book: Book) -> bool;
        fn add_member(&mut self, member: Member) -> bool;
        fn borrow_book(&mut self, member: Member, book: Book, duration: u16) -> bool;
    }
}
