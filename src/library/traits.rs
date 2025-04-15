use crate::library::types::Types::{
    Book,
    BookStatus,
    Member
};

pub mod Traits {
    use super::*;

    pub trait BookTrait {
        fn new(title: String, isbn: String, author: String, year: u64, status: BookStatus) -> Book;
    }
    
    pub trait LibraryTrait {
        fn add_book(library: &mut Self) -> bool;
        fn add_member(library: &mut Self) -> bool;
        fn borrow_book(member: Member, book: Book, duration: u16) -> bool;
        fn return_book(library: &mut Self, title: String) -> bool;
        fn search_book(library: &mut Self, title: String) -> Book;
    }
    
}