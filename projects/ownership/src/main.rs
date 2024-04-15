fn main() {
   let greeting = String::from("Hello, there");

   let (s, len) = str_get_len(greeting);

   println!("The length of '{}' is {}", s, len);
}

fn str_get_len(s: String) -> (String, usize){
    let len = s.len();

    return (s, len);
}


