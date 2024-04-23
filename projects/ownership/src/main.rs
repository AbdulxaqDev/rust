fn main() {
    let mut str = String::from("Brendan Eich");

    get_len(&mut str);

    println!("The length of '{}'", str);

    let mut value = 42;

    // First immutable borrow
    let reference1 = &value;
    println!("Immutable reference 1: {}", reference1);

    // Second immutable borrow
    let reference2 = &value;
    println!("Immutable reference 2: {}", reference2);

    // Mutable borrow
    let mutable_reference = &mut value;
    *mutable_reference *= 2;
    println!("Mutable reference: {}", mutable_reference);
}

fn get_len(str: &mut String) -> usize {
    str.push_str(" change!");
    str.len()
}
