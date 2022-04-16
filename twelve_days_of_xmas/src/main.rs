fn main() {
    println!("Twelve Days Of Christmas\n");
    for (i, day) in DAYS.iter().enumerate() {
        println!("On the {day} day of Christmas");
        println!("My true love sent to me");
        if i == 0 {
            println!("A partridge in a pear tree");
        } else {
            for gift in GIFTS[..=i].iter().rev() {
                println!("{gift}");
            }
        }
        println!();
    }
}

const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "And a partridge in a pear tree",
    "Two turtle doves",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];
