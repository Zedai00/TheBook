use std::{io::Write};

fn main() {
    println!("Fibonacci Generator");

    loop {
        print!("Enter a number: ");
        std::io::stdout().flush().unwrap();

        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("Error: Unable to read the input");

        let n: u64 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("Fibonacci Number: {}", fib(n));
        break;
    }
}

fn fib(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
