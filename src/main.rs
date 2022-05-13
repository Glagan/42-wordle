use std::fs;

use colored::Colorize;

fn main() {
    let mut list: Vec<char> = Vec::new();
    list.push('A');
    list.push('B');
    println!(
        "{} {:#?}",
        "Hello, world!".underline().on_bright_red(),
        list
    );

    let content = fs::read_to_string("resources/words.txt");
    for line in content.unwrap().split("\n") {
        println!("line {}", line);
    }
}
