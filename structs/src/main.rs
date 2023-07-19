struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64, 
    
}

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // struct update syntax
    // struct update syntax uses = like an assignment; this is because it moves the data just like we saw in  “Variables and Data Interacting with Move” section
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    user1.email = String::from("anotheremail@example.com");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// field init shorthand syntax

fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}