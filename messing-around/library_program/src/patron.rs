use crate::book::Book;

pub struct Patron {
    pub name: String,
    pub id: u32,
    pub owes_fee: bool,
    pub books: Vec<Book>,
}

impl Patron {
    pub fn new(name: String, id: u32) -> Patron {
        Patron {
            name,
            id,
            owes_fee: false,
            books: Vec::new(),
        }
    }
    
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    
    pub fn remove_book(&mut self, book: Book) {
        if let Some(index) = self.books.iter().position(|x| *x == book) {
            self.books.swap_remove(index);
        }
    }
}