// Convert temperatures between Fahrenheit and Celsius
use std::io;

fn main() {
    println!("--- Temperature converter ---");
    
    'main_loop: loop {
        let unit: char = get_unit();
        if unit == 'q' { break 'main_loop };
        
        let degrees: f64 = get_temp();
        
        /*
        if unit == "c" {
            println!("{degrees}c is {}f.", c_to_f(degrees));
        } else {
            println!("{degrees}f is {}c.", f_to_c(degrees));
        }
        */
        
        // Use match instead of if/else block above:
        match unit {
            'c' => println!("{degrees}c is {}f.", c_to_f(degrees)),
            'f' => println!("{degrees}f is {}c.", f_to_c(degrees)),
            _ => unreachable!(), // This tells compiler this (and all non-c-f-q branches) are unreachable
        }
    }
    
    println!("Goodbye!");
}

fn get_unit() -> char {
    loop {
        println!("Please enter 'c' or 'f' for temp you're entering (or 'q' to quit): ");
        
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        
        match input.trim().to_lowercase().parse::<char>() {
            Ok('c') => break 'c',
            Ok('f') => break 'f',
            Ok('q') => break 'q',
            _ => println!("Invalid unit! Please type 'c' or 'f'."),
        }
    }
}

fn get_temp() -> f64 {
    loop {
        println!("Please enter the temperature: ");
        
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read degrees!");
        
        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That wasn't a number!");
                continue;
            }
        };
        
        break input;
    }
}

fn c_to_f(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
