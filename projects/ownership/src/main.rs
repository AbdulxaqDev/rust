fn main() {
   let s = String::from("Hello, there");
    changes(&s);
}

fn changes(s: &String) {
    s.push_str("Hello there");
}


