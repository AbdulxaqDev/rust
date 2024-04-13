fn main() {
    let a = muliply_by2(4);
    five();

    println!("a is {a}")
}

fn five() -> u8 {
    5
}

fn muliply_by2(num: i128) -> i128 {
    num*2
}
