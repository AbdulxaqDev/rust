fn main() {
    let _s1 = gives_ownership();

    let s2 = String::from("Hello");

    let _s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let s = String::from("hehe");
    s    
}

fn takes_and_gives_back(s: String) -> String {
    s
}
