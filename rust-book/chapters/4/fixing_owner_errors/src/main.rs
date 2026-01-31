fn main() {
    let string1: String = String::from("Example string");
    let string2: String = String::from("Another string!");
    let vec_string: Vec<String> = vec![string1, string2];
    println!("{:?}", vec_string);
    let total: String = vec_string.join("...");
    println!("{total}");
    
    return;
}

// This doesn't compile because it returns a reference
// to a variable that doesn't outlive the function
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}