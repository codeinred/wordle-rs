use std::vec::Vec;

pub enum GuessState {
    Wrong,     // Indicates that the letter isn't in the word
    Elsewhere, // Indicates that the letter is found elsewhere in the word
    Right,     // Indicates that the letter is in the right place in the word
}
pub type Word = Vec<char>;
pub type Check = Vec<(char, GuessState)>;

pub struct Solution {
    word: Word,
}

impl Solution {
    pub fn check(&self, guess: &Word) -> Option<Check> {
        use GuessState::*;

        if self.word.len() != guess.len() {
            return None;
        }
        let result = self.word.iter().zip(guess).map(|(word_ch, guess_ch)| {
            if word_ch == guess_ch {
                (*word_ch, Right)
            } else {
                (*guess_ch, Wrong)
            }
        });

        Some(result.collect())
    }
}

pub fn hello_world() {
    println!("Hello, world!")
}
