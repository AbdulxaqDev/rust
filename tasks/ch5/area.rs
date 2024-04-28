fn main() {
    let w = 30;
    let h = 50;

    println!("The area is {}", area(w, h));
}

fn area(w: i32, h: i32) -> i32 {
    w*h
}