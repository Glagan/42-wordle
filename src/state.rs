use crate::{dictionary::Dictionary, guess::Guess};

pub struct State {
    dictionary: Dictionary,
    pub word: String,
    pub guesses: Vec<Guess>,
}

pub enum InputResult {
    Invalid,
    Correct,
    Incorrect,
    GameOver,
}

impl State {
    pub fn new(dictionary: Dictionary) -> State {
        State {
            dictionary,
            word: "".to_string(),
            guesses: vec![],
        }
    }

    pub fn restart(&mut self) -> Result<(), String> {
        self.random_new_word()?;
        self.guesses = vec![];
        Ok(())
    }

    pub fn guess_input(&mut self, input: String) -> InputResult {
        if !self.dictionary.words.contains(&input) {
            return InputResult::Invalid;
        }
        self.guesses.push(Guess::from_test(&self.word, &input));
        if self.word == input {
            return InputResult::Correct;
        }
        if self.guesses.len() == 6 {
            return InputResult::GameOver;
        }
        InputResult::Incorrect
    }

    pub fn guesses_left(&self) -> usize {
        6 - self.guesses.len()
    }

    fn random_new_word(&mut self) -> Result<(), String> {
        let word = self.dictionary.random_word();
        if let None = word {
            return Err("No word found in Dictionary !".to_string());
        }
        self.word = word.unwrap().to_string();
        Ok(())
    }

    pub fn print(&self) {
        for index in (0 as usize)..6 {
            let guess = self.guesses.get(index);
            if let Some(guess) = guess {
                guess.print();
                println!();
            } else {
                println!("_ _ _ _ _");
            }
        }
    }
}
