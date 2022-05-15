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
        let mut already_missplaced: Vec<usize> = vec![];
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
            else if word.contains(*letter) {
                let should_mark_missplaced = word.chars().enumerate().find(|(i, l)| {
                    !already_missplaced.contains(i)
                        && *l == *letter
                        && !guess.correct_letters.contains(i)
                        && input_chars[*i] != *l
                });
                if let Some((missplaced_index, _)) = should_mark_missplaced {
                    guess.missplaced_letters.push(index);
                    already_missplaced.push(missplaced_index);
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

#[test]
fn check_mixed_guess_1() {
    let guess = Guess::from_test("ERASE", "SPEED");
    assert_eq!(guess.correct_letters, vec![]);
    assert_eq!(guess.missplaced_letters, vec![0, 2, 3]);
}

#[test]
fn check_mixed_guess_2() {
    let guess = Guess::from_test("ABIDE", "SPEED");
    assert_eq!(guess.correct_letters, vec![]);
    assert_eq!(guess.missplaced_letters, vec![2, 4]);
}

#[test]
fn check_mixed_guess_3() {
    let guess = Guess::from_test("STEAL", "SPEED");
    assert_eq!(guess.correct_letters, vec![0, 2]);
    assert_eq!(guess.missplaced_letters, vec![]);
}

#[test]
fn check_mixed_guess_4() {
    let guess = Guess::from_test("CREPE", "SPEED");
    assert_eq!(guess.correct_letters, vec![2]);
    assert_eq!(guess.missplaced_letters, vec![1, 3]);
}

#[test]
fn check_mixed_guess_5() {
    let guess = Guess::from_test("LEPRA", "LERPS");
    assert_eq!(guess.correct_letters, vec![0, 1]);
    assert_eq!(guess.missplaced_letters, vec![2, 3]);
}

#[test]
fn check_mixed_guess_6() {
    let guess = Guess::from_test("DEOXY", "DANCE");
    assert_eq!(guess.correct_letters, vec![0]);
    assert_eq!(guess.missplaced_letters, vec![4]);
}

#[test]
fn check_mixed_guess_7() {
    let guess = Guess::from_test("DEOXY", "STIRS");
    assert_eq!(guess.correct_letters, vec![]);
    assert_eq!(guess.missplaced_letters, vec![]);
}

#[test]
fn check_mixed_guess_8() {
    let guess = Guess::from_test("DEOXY", "SKIED");
    assert_eq!(guess.correct_letters, vec![]);
    assert_eq!(guess.missplaced_letters, vec![3, 4]);
}

#[test]
fn check_mixed_guess_9() {
    let guess = Guess::from_test("ZOWEE", "EEVEN");
    assert_eq!(guess.correct_letters, vec![3]);
    assert_eq!(guess.missplaced_letters, vec![0]);
}

#[test]
fn check_mixed_guess_10() {
    let guess = Guess::from_test("ZOWEE", "ELPEE");
    assert_eq!(guess.correct_letters, vec![3, 4]);
    assert_eq!(guess.missplaced_letters, vec![]);
}

#[test]
fn check_correct_guess_1() {
    let guess = Guess::from_test("TIMES", "TIMES");
    assert_eq!(guess.correct_letters, vec![0, 1, 2, 3, 4]);
    assert_eq!(guess.missplaced_letters, vec![]);
}

#[test]
fn check_correct_guess_2() {
    let guess = Guess::from_test("ERASE", "ERASE");
    assert_eq!(guess.correct_letters, vec![0, 1, 2, 3, 4]);
    assert_eq!(guess.missplaced_letters, vec![]);
}
