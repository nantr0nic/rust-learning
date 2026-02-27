#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn valid_width(&self) -> bool {
        self.width > 0
    }
    
    fn width(&self) -> u32 {
        self.width
    }
    
    fn can_hold(&self, other_rec: &Rectangle) -> bool {
        if other_rec.area() < self.area() {
            true
        } else {
            false
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    
    println!("rect1 is {rect1:?}");
    //dbg!(println!("{rect1:#?}"));
    dbg!(&rect1);
    
    println!(
        "The area of the rect1 is {} square pixels.",
        area(&rect1)
    );
    
    println!(
        "The area of the rect1 is {} square pixels.",
        rect1.area()
    );
    
    println!("Rect1's width is valid: {}\nIts size: {}", rect1.valid_width(), rect1.width());
    
    // --------------------- //
    
    let rect2 = Rectangle{ width: 20, height: 40 };
    let rect3 = Rectangle{ width: 60, height: 60 };
    
    println!(
        "rect1 can hold rect2: {}\nrect2 can hold rect3: {}\nrect3 can hold rect1: {}", 
        rect1.can_hold(&rect2), rect2.can_hold(&rect2), rect3.can_hold(&rect1)
    );
    
}