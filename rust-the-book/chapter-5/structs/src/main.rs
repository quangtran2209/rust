fn main() {
    let user1 = User {
        email: String::from("user1@test.com"),
        username: String::from("user1"),
        sign_in_count: 100,
        active: true
    };

    println!("User1 = {:#?}", user1);

    let mut user2 = User {
        email: String::from("user2@test.com"),
        username: String::from("user2"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("user22@test.com");

    println!("User2 = {:#?}", user2);

    let user3 = build_user(
        String::from("user3@test.com"),
        String::from("user3")
    );

    println!("User3 = {:#?}", user3);

    let user4 = User {
        email: String::from("user4@test.com"),
        username: String::from("user4"),
        ..user3
    };

    println!("User4 = {:#?}", user4);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
