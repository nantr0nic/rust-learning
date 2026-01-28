// What is ownership in Rust?

fn main() {
    let first: String = String::from("Andy");
    // vvv This doesn't compile, just playing around and checking out errors
    // add_surname(first);
    // println!("Hello, {}!", first);
    // vvv this will work
    let full_name = add_surname(first);
    println!("Hello, {full_name}!");
}

fn add_surname(mut name: String) -> String {
    name.push_str(" Khazanovsky" );
    name
}