#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GuessState {
    Wrong,     // Indicates that the letter isn't in the word
    Elsewhere, // Indicates that the letter is found elsewhere in the word
    Right,     // Indicates that the letter is in the right place in the word
}

#[derive(PartialEq, Eq, Clone)]
pub struct Word(Vec<char>);

#[derive(PartialEq, Eq, Clone)]
pub struct Check(Vec<(char, GuessState)>);

impl From<&str> for Word {
    fn from(string: &str) -> Word {
        Word(string.chars().collect())
    }
}
impl From<String> for Word {
    fn from(string: String) -> Word {
        Word(string.chars().collect())
    }
}
impl From<&Word> for String {
    fn from(Word(word): &Word) -> String {
        word.iter().collect()
    }
}

impl Check {
    pub fn all_good(&self) -> bool {
        let Check(val) = self;
        val.iter().all(|(_, state)| *state == GuessState::Right)
    }
}

impl Word {
    pub fn check(&self, Word(guess): &Word) -> Option<Check> {
        use GuessState::*;

        let Word(word) = self;
        if word.len() != guess.len() {
            return None;
        }
        let result = word.iter().zip(guess).map(|(word_ch, guess_ch)| {
            if word_ch == guess_ch {
                (*guess_ch, Right)
            } else if word.contains(guess_ch) {
                (*guess_ch, Elsewhere)
            } else {
                (*guess_ch, Wrong)
            }
        });

        Some(Check(result.collect()))
    }
}
