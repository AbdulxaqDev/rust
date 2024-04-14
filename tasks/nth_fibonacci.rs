use std::io;

fn main() {
    let mut nth = String::new();

    println!("\nInput a number to get nth fibonacci number:");

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read the line!");

    let nth: u128 = nth.trim().parse().expect("Please input a number!");

    println!(
        "The {}{} fibonacci number is {}",
        nth,
        if nth == 1 {
            "st"
        } else if nth == 2 {
            "nd"
        } else if nth == 3 {
            "rd"
        } else {
            "th"
        },
        nth_fobinacci(nth)
    );
}

fn nth_fobinacci(n: u128) -> u128 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    }

    let mut nth: u128 = n - 2;
    let mut n1: u128 = 0;
    let mut n2: u128 = 1;
    let mut n3: u128 = 0;

    while nth > 0 {
        n3 = n1 + n2;
        n1 = n2;
        n2 = n3;
        nth -= 1;
    }
    return n3;
}
