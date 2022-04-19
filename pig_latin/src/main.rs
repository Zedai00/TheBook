use std::{
    io::{self, Write},
    vec,
};

fn main() {
    println!("Pig Latinizer");

    let mut text = String::new();
    print!("Input a word: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text).unwrap();

    let mut chars = text.trim().chars();
    let first = chars.next().unwrap();
    let text = chars.as_str();

    let result = piglatinize(first, text);
    println!("Result: {}", result);
}

fn piglatinize(first: char, text: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    if vowels.contains(&first) {
        format!(r"{}{}-hay", first, text)
    } else {
        format!(r"{}-{}ay", text, first.to_lowercase())
    }
}
