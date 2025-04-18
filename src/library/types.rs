pub mod Types {
    #[derive(Debug, Clone)]
    pub struct Member {
        pub name: String,
        pub id: u16,
        pub phone_no: String,
        pub address: String,
    }

    #[derive(Debug, Clone)]
    pub struct Book {
        pub title: String,
        pub isbn: String,
        pub author: String,
        pub year: u64,
        pub status: BookStatus,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum BookStatus {
        Available,
        Borrowed,
        UnderMaintenance,
    }

    #[derive(Debug)]

    pub enum MembershipType {
        Student,
        NonStudents,
        Staff,
    }

    pub struct Library<T, U> {
        pub books: Vec<T>,
        pub members: Vec<U>,
    }
}
