struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}

fn main() {
    let u1 = User {
        username: String::from("someone1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true
    };

    let u2 = build_user("someone1@example.com".to_string(), "someone2".to_string());

    let u3 = User {
        username: "someone3".to_string(),
        email: "someone2@example.com".to_string(),
        ..u2
    };
}