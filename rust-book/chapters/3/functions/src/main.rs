fn main() {
    print_labeled_measurement(5, 'h');
    
    // Statements and Expressions
    
    // Statements are "instructions that perform some action and do NOT return a value"
    // Expressions evaluate to a resultant value.
    
    let a = 5; // 5 is an expression that evaluates to 5
    // expressions can be part of statements
    let b = 4 + 1; // (4 + 1) is an expression because it returns a value
    // vvv won't compile vvv
    // let x = (let y = 6);
    let y : i32;
    let x = y = 7;
    println!("x: {:?}, y: {y}", x); // prints x: (), y: 7
    
    // -------------------------- //
    
    let a = {
        let x = 3;
        x + 2 // <-- this is what returns so no semi-colon
    };
    println!("a is {a}");
    
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}