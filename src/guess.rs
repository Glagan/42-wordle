use colored::Colorize;

pub struct Guess {
    pub word: String,
    pub correct_letters: Vec<usize>,
    pub missplaced_letters: Vec<usize>,
}

impl Guess {
    pub fn from_test(word: &str, input: &str) -> Guess {
        let mut guess = Guess {
            word: input.to_string(),
            correct_letters: vec![],
            missplaced_letters: vec![],
        };
        let word_chars: Vec<char> = word.chars().collect();
        for (index, letter) in input.chars().enumerate() {
            if *word_chars.get(index).unwrap() == letter {
                guess.correct_letters.push(index)
            }
            // TODO fix missplaced letters to ignore the letter if it was already marked missplaced
            else if word.contains(letter) {
                guess.missplaced_letters.push(index);
            }
        }
        guess
    }

    pub fn print(&self) {
        for (index, letter) in self.word.chars().enumerate() {
            if self.correct_letters.contains(&index) {
                print!("{} ", format!("{}", letter).bright_green());
            } else if self.missplaced_letters.contains(&index) {
                print!("{} ", format!("{}", letter).bright_yellow());
            } else {
                print!("{} ", letter);
            }
        }
    }
}
