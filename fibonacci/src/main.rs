use std::io;

fn main() {
    println!("Enter the nth fibonacci to computer:");
    let mut x = String::new();

    io::stdin().read_line(&mut x)
        .expect("Unable to read input");


    loop {
        let x = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fibonacci(x));
        break;
    }
}

fn fibonacci(x: u32) -> u32 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}
