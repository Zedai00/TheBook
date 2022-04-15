use std::io::{self, Write};

fn main() {
    println!("Temperature Converter");

    loop {
        println!("Input c to change from Celsius or f to change from Fahrenheit");
        print!("Input: ");
        io::stdout().flush().unwrap();
        let mut method = String::new();
        io::stdin()
            .read_line(&mut method)
            .expect("Error: Unable to read line");

        match method.as_str() {
            "f\n" => println!("You chose Fahrenheit"),
            "c\n" => println!("You chose Celsius"),
            _ => {
                println!("Please Enter Either f or c");
                continue;
            }
        };

        print!("Please Enter Temperature: ");
        io::stdout().flush().unwrap();
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Error: Unable to read input");
        let temp: i32 = temp.trim().parse().unwrap();

        match method.as_str() {
            "f\n" => {
                println!("{} Fahrenheit = {} Celsius", temp, ftoc(temp));
                break;
            }
            "c\n" => {
                println!("{} Celsius = {} Fahrenheit", temp, ctof(temp));
                break;
            }
            _ => unreachable!(),
        };
    }
}

fn ftoc(temp: i32) -> i32 {
    (temp - 32) * 5 / 9
}

fn ctof(temp: i32) -> i32 {
    (temp * 9 / 5) + 32
}
