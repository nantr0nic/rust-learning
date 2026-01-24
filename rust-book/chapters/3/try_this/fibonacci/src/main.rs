

use std::io;

fn main() {
    println!("--- Find nth Fibonacci Number ---");
    loop {
        println!("Please enter the nth Fibonacci you want: ");

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u128 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("That wasn't a number.");
                continue;
            }
        };

        println!("The {input}th Fibonacci number is: {}", fib_number_2(input));
    }
}

// More complex, slow 
fn fib_number(num: u128) -> u128 {
    if num <= 1 {
        return num;
    }
    return fib_number(num - 1) + fib_number(num - 2);
}

// Very fast
fn fib_number_2(num: u128) -> u128 {
    if num <= 1 { return num; }

    let mut prev1: u128 = 1;
    let mut prev2: u128 = 0;

    for _ in 2..=num {
        let current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}
