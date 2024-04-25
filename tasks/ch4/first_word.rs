use std::io;

fn main() {
    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read the line!");

    println!("->{}<-", first_word(&str));
}

fn first_word(s: &String) -> &str {
    let mut index = s.len();
    for (i, letter) in s.char_indices() {
        if letter == ' ' {
            index = i;
            break;
        }
    }

    &s[0..index]
}
