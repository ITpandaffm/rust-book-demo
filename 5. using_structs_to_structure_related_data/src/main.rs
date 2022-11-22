fn main() {

    println!("Hello, world!");

    // let mut user1 = User {
    //     email: String::from("hahaha@aaa.com"),
    //     username: String::from("hahaha"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // user1.username = String::from("ffm");


    // println!("user email:: {}", user1.email); 
    // println!("user username:: {}", user1.username);


    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user2 email:: {}", user2.email);
    println!("user111, {}", user1.email);

    println!("user2 name::{}", user2.username);
    println!("user111, {}", user1.username);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}