struct Rectangle {
    w: i32,
    h: i32,
}

fn main() {
    let rect1 = Rectangle { w: 30, h: 50 };

    println!("The area is {}", area(&rect1));
}

fn area(rect: &Rectangle) -> i32 {
    rect.w * rect.h
}
