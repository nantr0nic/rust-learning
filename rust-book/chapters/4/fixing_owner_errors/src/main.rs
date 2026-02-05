fn transfer(from: &mut i32, to: &mut i32, amount: i32) {
    *from -= amount;
    *to += amount;
}

fn print_info((id, name, amount): &(i32, String, i32)) {
    println!("ID: {id}\nName: {name}\nBalance: ${amount}");
}

fn main() {
    let string1: String = String::from("Example string");
    let string2: String = String::from("Another string!");
    let vec_string: Vec<String> = vec![string1, string2];
    println!("{:?}", vec_string);
    let total: String = vec_string.join("...");
    println!("{total}");
    
    let mut account_001: (i32, String, i32) = (001, String::from("Andy"), 1000);
    let mut account_002: (i32, String, i32) = (002, String::from("David"), 500);
    
    print_info(&account_001);
    print_info(&account_002);
    transfer(&mut account_001.2, &mut account_002.2, 200);
    print_info(&account_001);
    print_info(&account_002);
    
    let mut vec_numbers: Vec<i32> = vec![];
    
    // The "C or C++ habit"
    let mut i: i32 = 0;
    while i != 100 {
        vec_numbers.push(i);
        i += 1;
    }
    
    // The "Rust Way" -- using iterators/ranges (faster, safer)
    for i in 100..200 {
        vec_numbers.push(i);
    }
    
    println!("vec_numbers size: {}", vec_numbers.len());
    println!("#0: {}, #55: {}, #199: {}", vec_numbers[0], vec_numbers[55], vec_numbers[199]);
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