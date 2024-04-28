#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Point(i32, i32, i32);
struct Color(u8, u8, u8);

fn main() {
    // STRUCTS
    {
        // mutable struct
        let mut user1 = User {
            active: true,
            username: String::from("brendan"),
            email: String::from("brendan.foo@nasa.moon"),
            sign_in_count: 2,
        };

        // println!("{:#?}", user1);

        let user2 = User {
            email: String::from("new_user@nasa.mars"),
            ..user1
        };

        println!("{:#?}", user2);
        println!("user email {}", user1.sign_in_count);

        // field init syntax
        println!(
            "{:#?}",
            build_user(String::from("jony"), String::from("jony@nasa.moon"))
        );
    }
    
    // TUPLE STRUCTS
    {

        let p1 = Point(3, 4, 5);
        let p2 = Point(5, 4, 3);

        let color1 = Color(255, 255, 255);

        println!("Distance between p1 and p2 is {}", distance(p1, p2));
        println!("Distance between p1 and p2 is {}", distance(color1, color1));
    }
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // we are not doing "username: username" because they are the same
        email,    // the same here
        sign_in_count: 1,
    }
}


fn distance(p1: Point, p2: Point) -> i32 {
    (
        (p1.0 + p2.0)^2 +
        (p1.1 + p2.1)^2 +
        (p1.2 + p2.2)^2
    )^(1/2)
}