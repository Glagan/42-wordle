use crate::{
    dictionary::Dictionary,
    state::{InputResult, State},
};
use colored::Colorize;
use std::{
    io::{self, Write},
    process,
};
use tracker::Tracker;

mod dictionary;
mod guess;
mod state;
mod tracker;

fn input_line(prefix: &str) -> Result<String, String> {
    print!("{}", prefix);
    io::stdout().flush().unwrap();
    let mut user_input: String = String::new();
    let result = io::stdin().read_line(&mut user_input);
    if let Err(error) = result {
        Err(error.to_string())
    } else {
        Ok(user_input.trim().to_string().to_uppercase())
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
    // Setup state
    let mut tracker = Tracker::new();
    let mut state = State::new(dictionary);

    // Main loop
    let mut do_loop = true;
    while do_loop {
        // Generate a new word and clean previous guesses
        state.restart().unwrap_or_else(|error| {
            eprintln!("Failed to start a game: {}", format!("{}", error).red());
            process::exit(1);
        });

        // Debug
        println!(">> The word is {}", state.word.blue());

        // Input loop
        let mut do_input = true;
        while do_input {
            // Ask for input
            let user_input = input_line("input: ").unwrap_or_else(|error| {
                eprintln!(
                    "{}",
                    format!("Failed to read input: {}", error.to_string()).red()
                );
                process::exit(1);
            });

            // Check if it's valid
            if user_input.len() != 5 {
                eprintln!("{}", "Entered words must be of length 5".yellow());
                continue;
            }

            // Guess from state
            let result = state.guess_input(user_input.to_uppercase());
            state.print();
            match result {
                InputResult::Invalid => {
                    println!(
                        "{} {} {}",
                        "The word".yellow(),
                        format!("{}", user_input).blue(),
                        "is not in the dictionary".yellow()
                    );
                }
                InputResult::GameOver => {
                    println!(
                        "{} The word was {} !",
                        "You ran out of guesses !".purple(),
                        state.word.green()
                    );
                    tracker.add_game(false);
                    do_input = false;
                }
                InputResult::Correct => {
                    println!(
                        "You guessed correctly ! The word was {} !",
                        state.word.green()
                    );
                    tracker.add_game(true);
                    state.print_emoji();
                    do_input = false;
                }
                InputResult::Incorrect => {
                    println!(
                        "That's not the word, you have {} guesses left.",
                        format!("{}", state.guesses_left()).blue()
                    );
                }
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

    // Show the session summary
    tracker.print_summary();
}
