use crate::library::types::Types::{
    Book,
    BookStatus,
    Member
};
use crate::library::traits::Traits::{BookTrait};


impl BookTrait for Book {
    fn new(title: String, isbn: String, author: String, year: u64, status: BookStatus) -> Book {
        
    }
}


