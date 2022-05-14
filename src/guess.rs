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
        let mut already_missplaced: Vec<char> = vec![];
        let input_chars: Vec<char> = input.chars().collect();
        let word_chars: Vec<char> = word.chars().collect();
        for (index, letter) in input_chars.iter().enumerate() {
            if *word_chars.get(index).unwrap() == *letter {
                guess.correct_letters.push(index)
            }
            // -- with {} missplaced and [] correct
            // e.g word: zowee input: eeven
            // -- should resolve {e}ev[e]n
            // and word: zowee input: elpee
            // -- should resolve elp[e][e]
            else if word.contains(*letter) && !already_missplaced.contains(&letter) {
                let should_mark_missplaced = word.chars().enumerate().any(|(i, l)| {
                    l == *letter && !guess.correct_letters.contains(&i) && input_chars[i] != l
                });
                if should_mark_missplaced {
                    guess.missplaced_letters.push(index);
                    already_missplaced.push(*letter);
                }
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
                print!("{} ", format!("{}", letter).dimmed());
            }
        }
    }

    pub fn print_emoji(&self) {
        for (index, _) in self.word.chars().enumerate() {
            if self.correct_letters.contains(&index) {
                print!("🟩");
            } else if self.missplaced_letters.contains(&index) {
                print!("🟨");
            } else {
                print!("⬛");
            }
        }
    }
}
