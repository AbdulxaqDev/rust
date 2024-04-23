use std::io;

fn main() {
    let mut str = String::new();
    let mut f_word = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read the line!");


    for letter in str.chars() {
        f_word.push(letter);
        if letter == ' ' {
            break;
        }
    }

    println!("First word: {}", f_word);
}
