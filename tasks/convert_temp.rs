use std::io;

fn main() {
    let mut n = String::new();
    let mut convert_to = String::new();

    println!("Enter 'f' if you want to conver to Fahrenheit to Temperature, F -> T");
    println!("Enter 't' if you want to conver Fahrenheit to Temperature, T -> F");
    io::stdin()
        .read_line(&mut convert_to)
        .expect("Failed to read line! ");

    println!(
        "Enter {}:",
        if convert_to == "t" {
            "temperature number"
        } else {
            "fahrenheit number"
        }
    );

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line! ");

    let n: f64 = n.trim().parse().expect("Please input number! ");

    println!("The {}°F is {:.1}°C", n, convert(n, convert_to == "t"));
}

fn convert(n: f64, is_celcium: bool) -> f64 {
    return if is_celcium {
        (n - 32.0) * (5.0 / 9.0)
    } else {
        n * (9.0 / 5.0) + 32.0
    };
}
