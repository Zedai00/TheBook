use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    // let mut numbers = vec![5, 1, 4, 884, 955, 54, 45, 45, 65, 65, 54, 2, 46, 83];
    println!("Calculate Median and Mode");
    print!("Input numbers split by spaces: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("Media: {}", median(&mut input));
    let (num, count) = mode(&mut input);
    println!("Mode number: {}, Count: {}", num, count);
}

fn mode(input: &mut str) -> (i32, i32) {
    let mut numbers = HashMap::new();
    let mut count = 0;
    let mut num = 0;
    for i in input.trim().split(' ') {
        let n: i32 = i.parse().unwrap();
        let counter = numbers.entry(n).or_insert(0);
        *counter += 1;
        if *counter > count {
            count = *counter;
            num = n;
        }
    }
    (count, num)
}

fn median(input: &mut str) -> i32 {
    let mut numbers = Vec::new();
    for i in input.trim().split(' ') {
        let n: i32 = i.parse().unwrap();
        numbers.push(n);
    }

    let len = numbers.len();
    numbers.sort_unstable();

    if numbers.len() % 2 == 0 {
        // If length is even then median is two of the middle numbers added and divided by 2
        (numbers[(len / 2) - 1] + numbers[len / 2]) / 2
    } else {
        numbers[numbers.len() / 2]
    }
}
