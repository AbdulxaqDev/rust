#[derive(Debug)]
struct Rectangle {
    w: i32,
    h: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.w * self.h
    }
}

fn main() {
    let rect1 = Rectangle { w: 30, h: 50 };

    println!("The area is {}", rect1.area());
}
