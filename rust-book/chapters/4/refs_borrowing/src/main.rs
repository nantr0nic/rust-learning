// References are non-owning pointers (to data)

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
    
    // Derefs
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    *x += 1;
    println!("a is 1, x is 2? a: {a} / x: {}", *x);
    
    let r1: &Box<i32> = &x;
    let b: i32 = **r1;
    
    let r2: &i32 = &*x;
    let c: i32 = *r2;
    
    // Aliasing and Mutation
    let mut v: Vec<i32> = vec![0, 1, 2, 3];
    let num: &i32 = &v[2];
    println!("{} is in {:?}", *num, v);
    // *num = *num + 1; <-- can't mutate (thru) an alias
    v.push(4);
    // println!("{} won't compile now", *num); <-- this causes the previous line to become an error
    println!("{:?}", v);
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}