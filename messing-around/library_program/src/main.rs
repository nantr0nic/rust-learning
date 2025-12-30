// This is just messing around, trying out an exercise Bjarne gave in C++ but in Rust
// Just looked at some example code and kind of guess-and-checking how it works. 
// Experimenting!

pub mod book;
pub mod patron;
pub mod catalogue;

use patron::Patron;
use book::Book;
use catalogue::Catalogue;

use std::io;

fn main() {
    println!("Library Management System");
    
    let mut main_catalogue = Catalogue::new();
    
    loop {
            println!("\n1. Add Patron");
            println!("2. Add Book");
            println!("3. List Books");
            println!("4. Check out book");
            println!("5. Check in book");
            println!("Enter choice:");
    
            let mut input = String::new();
    
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "1" => { println!("Adding Patron..."); add_patron(); },
                "2" => println!("Adding Book..."),
                "3" => { println!("Listing Books..."); main_catalogue.list_books(); },
                "4" => println!("Checking out..."),
                "5" => println!("Checking in..."),
                "q" | "quit" => break,
                _ => println!("Invalid option, please try again."), // default
            }
        }
}

fn add_patron() {
    
}

fn add_book() {
    
}
