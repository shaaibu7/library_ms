pub mod Types {
    pub struct Member {
        pub name: String,
        pub id: u16, 
        pub phone_no: String,
        pub address: String,
    
    }
    
    pub struct Book {
        pub title: String,
        pub isbn: String,
        pub author: String,
        pub year: u64,
        pub status: BookStatus,
    }
    
    pub enum BookStatus {
        Available,
        Borrowed,
        UnderMaintenance
    }
    
    pub enum MembershipType {
        Student,
        NonStudents,
        Staff
    }
    
    
    pub struct Library<T, U> {
        pub books: Vec<T>,
        pub members: Vec<U>
    }
}