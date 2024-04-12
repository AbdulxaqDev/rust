const NUM: i8 = 1_2_7;

fn main() {
    println!("constant NUM is {NUM}");

    let x = 5;

    println!("x  is {x}");

    let x = x + 18;

    println!("shadowed x  is {x}");

    let a_char = 'A';
    println!("{}", a_char);
}
