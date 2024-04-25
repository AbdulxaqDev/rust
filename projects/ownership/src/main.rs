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

    // ------

    let mut s = String::from("Tester tester");
    let word = get_len(&s);

    s.clear(); // error, because mutation happening before immutable usage
    
    println!("Tester: {:#?}", word);

    let full_name = String::from("Brendan Eich");

    let name = &full_name[0..7];
    let surname = &full_name[8..full_name.len()];

    println!("Name:-{}, Surname:-{}", name, surname);
}

fn get_len(s: &String) -> &str {
    &s[..3]
}
