// References are non-owning pointers (to data)

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
    
    // ----- Derefs -----
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    *x += 1;
    println!("a is 1, x is 2? a: {a} / x: {}", *x);
    
    let mut y: Box<[i32; 3]> = Box::new([1, 2, 3]);
    let box_array: [i32; 3] = *y;
    let array_elem: i32 = y[0];
    let ref1: &i32 = &y[1];
    let dref1: i32 = *ref1;
    let ref2: &[i32; 3] = &box_array;
    let dref2: &i32 = &ref2[2];
    let dref3: i32 = *dref2;
    println!("y: {:?} / b_a: {:?} / dref: {dref1} / dref3: {dref3}", y, box_array);
    
    let r1: &Box<i32> = &x;
    let b: i32 = **r1;
    
    let r2: &i32 = &*x;
    let c: i32 = *r2;
    
    // ----- Aliasing and Mutation ------
    let mut v: Vec<i32> = vec![0, 1, 2, 3];
    let num: &i32 = &v[2];
    println!("{} is in {:?}", *num, v);
    // *num = *num + 1; <-- can't mutate (thru) an alias
    v.push(4);
    // println!("{} won't compile now", *num); <-- this causes the previous line to become an error
    println!("{:?}", v);
    
    // ----- Borrow checker stuff -----
    let mut v: Vec<i32> = vec![1, 2, 3];    // v is +R +W +O
    v[2] = 6;
    let num: &i32 = &v[2];                  // v is R / -W -O, num is +R and +O, *num is +R
    // v[1] = 3; //<-- illegal b/c of num (borrowed value)
    println!("Third element is {}", *num);  // v regains RWO, num/*num lose RO / R
    v[1] = 4; // <-- legal AFTER num is used in the above line (num loses permissions on this line, v regains)
    v.push(4);                              // v loses RWO
    
    // Mutable references
    let mut v2: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v2[2];
    // v2[2] = 66; <-- still illegal BUT...
    *num = 66; // <-- this is not (and it changes v2 even though v2 does NOT have RWO here)
    println!("Third element (of v2) is {}", *num); // <-- must be here
    println!("Third element (of v2) is: {}", v2[2]);
    // println!("Third element (of v2) is {}", *num); // <-- can't be here if we want the line above
    //    ^^^ because v2 has lost read permissions while num is "in use"
    /*
        The phrase “in use” is describing a reference’s lifetime, or the range of code
        spanning from its birth (where the reference is created) to its death 
        (the last time(s) the reference is used).
    */
    v2[1] = 44;
    println!("v2 is: {:?}", v2);
    
    let mut xx = 1;
    let yy: &mut i32 = &mut xx; // xx loses READ
    let zz = *yy;
    *yy += 1;
    // println!("{xx}, {yy}, {zz}"); // thus this won't compile
    println!("yy: {yy}, zz: {zz}"); // but this will
    println!("xx: {xx}"); // and now this works because yy's lifetime ended
    
    /*
    
    -> All variables can read, own, and (optionally) write their data.
    -> Creating a reference will transfer permissions from the borrowed place to the reference.
    -> Permissions are returned once the reference’s lifetime has ended.
    -> Data must outlive all references that point to it.

    */
    
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}