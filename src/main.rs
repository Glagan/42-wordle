use crate::dictionary::Dictionary;
use colored::Colorize;
use std::{
    io::{self, Write},
    process,
};

mod dictionary;

fn input_line(prefix: &str) -> Result<String, String> {
    print!("{}", prefix);
    io::stdout().flush().unwrap();
    let mut user_input: String = String::new();
    let result = io::stdin().read_line(&mut user_input);
    if let Err(error) = result {
        Err(error.to_string())
    } else {
        Ok(user_input.trim().to_string())
    }
}

fn main() {
    // Load dictionary
    let dictionary = Dictionary::from_file("resources/words.txt");
    if let Err(error) = dictionary {
        eprintln!("{}", error.red());
        process::exit(1);
    }
    let dictionary = dictionary.unwrap();
    dictionary.check().unwrap_or_else(|error| {
        eprintln!("{}", error.red());
        process::exit(1);
    });

    // Main loop
    let mut do_loop = true;
    while do_loop {
        // Random word
        let word = dictionary.random_word().unwrap_or_else(|| {
            eprintln!("{}", "No word found in Dictionary !".red());
            process::exit(1);
        });

        // Debug
        println!("The word is {}", word.blue());

        // Input loop
        let mut do_input = true;
        while do_input {
            let user_input = input_line("input: ").unwrap_or_else(|error| {
                eprintln!(
                    "{}",
                    format!("Failed to read input: {}", error.to_string()).red()
                );
                process::exit(1);
            });
            println!("entered: {}", user_input);

            // Check if it's valid
            if user_input.len() != 5 {
                eprintln!("{}", "Entered words must be of length 5".yellow());
                continue;
            }

            // Check if it's the word
            if *word == user_input {
                println!("{}", "You found the word !".green());
                do_input = false;
            }
        }

        // Continue ?
        let do_continue = input_line("Do you want to play again ? [Y/n] ").unwrap_or_else(|_| {
            process::exit(0);
        });
        let user_input_command = do_continue.to_lowercase();
        if user_input_command == "n"
            || user_input_command == "no"
            || user_input_command == "q"
            || user_input_command == "quit"
            || user_input_command == "exit"
        {
            do_loop = false;
        }
    }
}
