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


    fn all_books(&self) ->Option<&Vec<Book>>
    {
        if &self.books.len() > &0 {
          return  Some(&self.books)
        }
        None
    }
            
    fn add_member(&mut self, member: Member) -> bool {
        if self.members.iter().any(|m| m.id == member.id) {
            return false;
        }

        self.members.push(member);
        true
    }


    fn borrow_book(&mut self, member: Member, book: Book) -> bool {
        //=== Check if member exists ===//
        if !self.members.iter().any(|m| m.id == member.id) {
            return false;
        }
        //=== Find book and change status ===//
        for bk in &mut self.books {
            if bk.isbn == book.isbn && bk.status == BookStatus::Available {
                bk.status = BookStatus::Borrowed;
                return true;
            }
        }

        false
    }

}
        


