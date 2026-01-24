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
    
    println!("3? -> {}", f(g(1))); // should return 3
    println!("5? -> {}", {
        let x: i32 = 3;
        f(g(x))
    })
    
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn f(x: i32) -> i32 {
    x + 1
}

fn g(x: i32) -> i32 {
    x + 1
}