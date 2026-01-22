#[derive(Debug)]
struct MyType {
    a: char,
    b: char
}

// impl MyType {
//     pub fn new(a: char, b: char) -> MyType {
//         MyType {
//             a,
//             b
//         }
//     }
// }

fn main() {
    // Scalar types (represent a single value)
    let a: u8 = 255;
    let b: i16 = -1024;
    let d: u16 = 65_535;
    let e: u32 = 2_600_000;
    let f: u64 = 68_000_000;
    let g: i128 = -999_999_999_999_999;
    let h: isize = -256_256_256_256;
    
    // Floating point types
    let c: f32 = 3.141;
    // also f64 (which is the default)
    println!("Scalar and Floating Point\nu8: {a} / i16: {b} / f32: {c} / u16: {d} /
        u32: {e} / u64: {f} / i128: {g} / isize: {h}");

    // Boolean and character
    let bb: bool = true;
    let cc: char = 'a';
    let cc2: char = '\u{65}';
    // char can store a variety of things
    let cc3: char = '😲';
    println!("Boolean and char\nbool: {bb} / char: {cc} / char: {cc2} / char: {cc3}");
    
    // Compound types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("Can you print the tuple?\nwith :? {:?} / with :#? {:#?}", tup, tup);
    let two_icons: MyType = MyType { a: '🙀', b: '🦀'};
    let tup_2: (i32, &MyType) = (42, &two_icons);
    let tup_3: (i32, &char, &char) = (24, &two_icons.a, &two_icons.b);
    println!("Tuple #2: {:?}", tup_2);
    println!("Tuple #3 emoji parts: {} & {}", tup_3.1, tup_3.2);
    
    // Array
    let arr_a = [1, 2, 3, 4, 5]; // defaults to i32
    let arr_specify: [u8; 5] = [1, 2, 3, 4, 5]; // specify u8
    let arr_init = ['🦀'; 5]; // an array of five 🦀's
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    // access like C++
    println!("arr_a[0]: {} / arr_specify: {:?} / arr_init[4]: {} / Born in: {}",
            arr_a[0], arr_specify, arr_init[4], months[1]);
}
