fn main() {
    let mut x: u16 = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("How long is three hours in seconds? It is: {THREE_HOURS_IN_SECONDS}");
    
    let y = 5;
    println!("The value of y is {}", y);
    let y = 6;
    println!("The value of y is {}", y);
    let y = y + 1;
    println!("The value of y is {}", y);
    
    {
        let y = x * y;
        println!("The value of y is {}", y);
    }
    
    println!("The value of y is {y}");
    
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);
    
    let mut spaces_2 = "     "; // useful warning here
    //spaces_2 = spaces_2.len();
    let spaces_2 = spaces_2.len();
    println!("The value of spaces_2 is {}", spaces_2);
}
