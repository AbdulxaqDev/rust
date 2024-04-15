fn main() {
   let g = String::from("Hello, there");

   let l = get_len(&g);

   println!("The length of '{}' is {}", g, l);
}

fn get_len(s: &String) -> usize{
    s.len()
}


