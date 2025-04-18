use crate::library::traits::traits::LibraryTrait;
use crate::library::types::Types::{Book, Library, Member};

impl LibraryTrait for Library<Book, Member> {
    fn add_book(&mut self, book: Book) -> bool {
        if self.books.iter().any(|b| b.isbn == book.isbn) {
            return false;
        }
        self.books.push(book);
        true
    }
}
