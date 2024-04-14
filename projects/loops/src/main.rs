fn main() {
    let mut number = 0;

    let end_loop_result = loop {
        number += 1;
        if number == 10 {
            break number;
        }
    };

    println!("end loop result {end_loop_result}");

    let mut material = 0;
    let target = 10;

    'factory: loop {
        let mut gas = 3;
        println!("Material amount: {material}");
        println!("Starting engine!");

        loop {
            println!("Gas left: {gas}");
            if gas == 0 {
                println!("Stopping engine!");
                break;
            }
            if material == target {
                println!("We got enough, stoping factory!");
                break 'factory;
            }
            gas -= 1;
            material += 1;
        }
    }
    println!("Material are made: {material}");

    let mut age = 0;

    while age < 9 {
        age += 1;
    }

    println!("wh: {age}");

    for n in (1..5).rev() {
        println!("N is {n}");
    }
}
