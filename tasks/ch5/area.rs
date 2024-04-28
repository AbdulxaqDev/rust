fn main() {
    let rect1 = (30, 50);

    println!("The area is {}", area(rect1));
}

fn area(rect: (i32, i32)) -> i32 {
    rect.0*rect.1
}