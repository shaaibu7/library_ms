use crate::library::traits::traits::LibraryTrait;
use crate::library::types::Types::{Book, BookStatus, Library, Member};

impl LibraryTrait for Library<Book, Member> {
    fn add_book(&mut self, book: Book) -> bool {
        if self.books.iter().any(|b| b.isbn == book.isbn) {
            return false;
        }
        self.books.push(book);
        true
    }

    fn add_member(&mut self, member: Member) -> bool {
        if self.members.iter().any(|m| m.id == member.id) {
            return false;
        }

        self.members.push(member);
        true
    }

    fn borrow_book(&mut self, member: Member, book: Book, _duration: u16) -> bool {
        //=== Check if member exists ===//
        if !self.members.iter().any(|m| m.id == member.id) {
            return false;
        }
        //=== Find book and change status ===//
        for book in &mut self.books {
            if book.isbn == book.isbn && book.status == BookStatus::Available {
                book.status = BookStatus::Borrowed;
                return true;
            }
        }

        false
    }
}
