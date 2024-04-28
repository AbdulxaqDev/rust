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

fn main() {
    let rect1 = Rectangle { w: 30, h: 50 };
    let rect2 = Rectangle { w: 60, h: 40 };

    println!("Can hold: {}", rect1.can_hold(&rect2));
}
