fn main() {
    let number = 81;

    if number > 5 {
        println!("number is greater than 5")
    } else if number < 5 {
        println!("number is less than 5")
    } else {
        println!("number is equal to {number}")
    }

    if number % 4 == 0 {
        println!("number is devisible by 4");
    } else if number % 3 == 0 {
        println!("number is devisible by 3");
    } else if number % 2 == 0 {
        println!("number is devisible by 2");
    } else {
        println!("number is not devisible by 2, 3 and 4");
    }

    let condition = false;
    let num = if condition { 5 } else { 7 };

    println!("num is {num}");
}
