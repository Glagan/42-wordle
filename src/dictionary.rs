use rand::seq::SliceRandom;
use std::fs;

pub struct Dictionary {
    pub length: usize,
    pub words: Vec<String>,
}

impl Dictionary {
    pub fn from_file(file_path: &str) -> Result<Dictionary, String> {
        let content = fs::read_to_string(file_path);
        if let Err(e) = content {
            return Err(e.to_string());
        }

        let mut words: Vec<String> = vec![];
        for line in content.unwrap().split("\n") {
            let clean_line = line.trim();
            if clean_line.len() == 5 {
                words.push(clean_line.to_string());
            }
        }

        Ok(Dictionary {
            length: words.len(),
            words,
        })
    }

    pub fn check(&self) -> Result<(), String> {
        if self.length == 0 {
            return Err("Empty Dictionary !".to_string());
        }

        Ok(())
    }

    pub fn random_word(&self) -> Option<&String> {
        self.words.choose(&mut rand::thread_rng())
    }
}
