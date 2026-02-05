fn original_first_word(s: &String) -> usize {
    /*
   Because we need to go through the String element by element and check whether 
   a value is a space, we’ll convert our String to an array of bytes using the 
   as_bytes method
    */
    let bytes = s.as_bytes();

    /*
   For now, know that iter is a method that returns each element in a collection 
   and that enumerate wraps the result of iter and returns each element as part 
   of a tuple instead. The first element of the tuple returned from enumerate is 
   the index, and the second element is a reference to the element. This is a bit 
   more convenient than calculating the index ourselves.
    */
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn sslice_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    let mut s = String::from("hello world");
    let word = sslice_first_word(&s);
    let word2 = original_first_word(&s);
    println!("{word}");
    println!("{}", &s[0..word2]);
    //s.clear(); // after here the index held by 'word' is useless... so --> string slices!
    
    // String slices
    let hello: &str = &s[..=5]; // =5 to include the space after 'hello'
    let world = &s[6..]; // compiler deduces &str type
    println!("{hello}{world}");
}
