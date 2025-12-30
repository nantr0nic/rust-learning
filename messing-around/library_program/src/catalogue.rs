use crate::book::Book;
use crate::patron::Patron;

pub struct Catalogue {
    catalogue: Vec<Book>,
    checked_out_books: Vec<Book>,
}

impl Catalogue {
    pub fn new() -> Catalogue {
        Catalogue { catalogue: Vec::new(), checked_out_books: Vec::new() }
    }
    
    pub fn add_book(&mut self, book: Book) {
        self.catalogue.push(book);
    }
    
    pub fn remove_book(&mut self, book: Book) {
        if let Some(index) = self.catalogue.iter().position(|x| *x == book) {
            self.catalogue.swap_remove(index);
        }
    }
    
    pub fn check_out_book(&mut self, book: &Book, patron: &Patron) {
        if let Some(index) = self.catalogue.iter().position(|x| *x == *book) {
            self.catalogue[index].check_out();
            self.checked_out_books.push(book.clone());
            patron.add_book(book);
            println!("{} checked out by {}", book.title, patron.name);
        }
    }
    
    pub fn check_in_book(&mut self, book: &Book, patron: &Patron) {
        if let Some(index) = self.checked_out_books.iter().position(|x| *x == *book) {
            self.checked_out_books.swap_remove(index);
            if let Some(index) = self.catalogue.iter().position(|x| *x == *book) {
                self.catalogue[index].check_in();
            }
            patron.remove_book(book);
            println!("{} checked in by {}", book.title, patron.name);
        }
    }
    
    pub fn list_books(&self) -> Vec<&Book> {
        self.catalogue.iter().collect()
    }
}
