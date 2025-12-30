#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub checked_out: bool,
}

impl Book {
    pub fn new(title: String, author: String) -> Book {
        Book { 
            title, 
            author,
            checked_out: false
        }
    }
    
    pub fn check_out(&mut self) {
        self.checked_out = true;
    }
    
    pub fn check_in(&mut self) {
        self.checked_out = false;
    }
}
