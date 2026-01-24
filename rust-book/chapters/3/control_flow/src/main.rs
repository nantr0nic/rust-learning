fn main() {
    // Control flow = expression
    
    // You can use if conditions in statements like this
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    
    // Intro of loop, can use it in statements
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // this semi-colon is optional
        }
    };

    println!("The result is {result}");
    
    // You can label loops and break out of them using their labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    
    // While loop
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    
    // for-range loop
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }
    
    // discrete loop using Range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}