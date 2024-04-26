#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // mutable struct
    let mut user1 = User {
        active: true,
        username: String::from("brendan"),
        email: String::from("brendan.foo@nasa.moon"),
        sign_in_count: 2,
    };

    user1.active = false;

    // println!("{:#?}", user1);

    let user2 = User {
        email: String::from("new_user@nasa.mars"),
        ..user1
    };

    println!("{:#?}", user2);

    // field init syntax
    println!(
        "{:#?}",
        build_user(String::from("jony"), String::from("jony@nasa.moon"))
    );
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // we are not doing "username: username" because they are the same
        email,    // the same here
        sign_in_count: 1,
    }
}
