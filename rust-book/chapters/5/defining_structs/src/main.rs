struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneblah1232323"),
        active: true,
        sign_in_count: 1,
    };
    
    user1.email = String::from("adifferentemailnow@email.com");
    
    let user2 = User {
        email: String::from("newnew@email.com"),
        ..user1
    };
    // ---------- //
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;
    println!("x,y,z = {x},{y},{z}");
    // ---------- //
    let subject = AlwaysEqual;
    
}
