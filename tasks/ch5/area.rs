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

impl Rectangle {
    fn w(&self) -> bool {
        self.w > 0
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

impl Rectangle {
    fn square(side: i32) -> Self {
        Self {
            w: side,
            h: side,
        }
    }
}

fn main() {
    let rect1 = Rectangle { w: 30, h: 50 };
    let rect2 = Rectangle { w: 60, h: 40 };

    let sqr1 = Rectangle::square(30);

    println!("Square: {:#?}", sqr1);
}
