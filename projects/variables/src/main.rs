use std::io;

fn main() {
    const MONTHES: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index.trim().parse().expect("Enter a number!");

    let element = MONTHES[index];

    println!("Month is {element}");
}
