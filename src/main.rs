mod library;

use crate::library::traits::traits::{BookTrait, LibraryTrait};
use crate::library::types::Types::{Book, BookStatus, Library, Member};

fn main() {
    // ==== Initialize a book =====//
    let book1 = Book::new(
        String::from("The Diary of a Wimpy Kid"),
        "1223-4432-4411".to_string(),
        "Jay B".to_string(),
        2005,
        BookStatus::Available,
    );

    // ===== Initialize an empty library ====== //
    let mut library: Library<Book, Member> = Library {
        books: Vec::new(),
        members: Vec::new(),
    };

    // ===Add the book using the trait method ====//
    let success = library.add_book(book1);

    if success {
        println!("Book added successfully!");
    } else {
        println!("Failed to add book!");
    }

    // ===Check the number of books in the library ===//
    println!("Number of books in library: {}", library.books.len());

    println!("{:?}", library.books[0]);

    let member = Member { name: "suleiman".to_owned(), id: (library.members.len() as u16) + 1, phone_no: "+2348012334311".to_owned(), address: "old airport".to_owned() };

    let member_success = library.add_member(member);

    let member2 = Member { name: "suleiman".to_owned(), id: (library.members.len() as u16) + 1, phone_no: "+2348012334311".to_owned(), address: "old airport".to_owned() };

    let member_success2 = library.add_member(member2);

    println!("The result of adding members to library is: {}", member_success);
    println!("The result of adding members to library is: {}", member_success2);

    println!("The members in this library is: {:?}", library.members[0]);
    println!("The members in this library is: {:?}", library.members[1]);


}
