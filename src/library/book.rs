
use crate::library::types::Types::{
    Book,
    BookStatus
};
use crate::library::traits::traits::BookTrait;

impl BookTrait for Book {
    fn new(title: String, isbn: String, author: String, year: u64, status: BookStatus) -> Book {
        Book {
            title,
            isbn,
            author,
            year,
            status,
        }
    }
}