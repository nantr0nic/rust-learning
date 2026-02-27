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

fn sslice_first_word(s: &str) -> &str {
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
    
    // Can also take slices with ranges like so:
    let len = s.len();
    let whole_string = &s[..len];
    // or like so
    let whole_string = &s[..];
    println!("{whole_string}");
    
    // ---------------- //
    
    let my_string = String::from("hello world");
    
    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = sslice_first_word(&my_string[0..6]);
    let word = sslice_first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = sslice_first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = sslice_first_word(&my_string_literal[0..6]);
    let word = sslice_first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = sslice_first_word(my_string_literal);
    
    // --------------------- //
    
    // You can also have slices of other types (covered in more detail ch 8)
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3];
    assert_eq!(a_slice, &[2, 3]);
}
