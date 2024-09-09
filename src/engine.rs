use crate::word_list::*;
use rand::prelude::SliceRandom;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SlotState {
    Match,
    WrongPos,
    NoMatch,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EngineState {
    Found,
    OutOfGuesses,
    Guessing,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LetterSlot {
    pub(crate) chr: char,
    pub(crate) state: SlotState,
}

#[derive(Debug, Default)]
pub struct SubmittedGuessInfo {
    pub word: String,
    pub matches: Vec<char>,
    pub mismatches: Vec<char>,
    pub no_matches: Vec<char>,
}

impl LetterSlot {
    pub fn new(chr: char, state: SlotState) -> Self {
        LetterSlot { chr, state }
    }
}

#[derive(Debug)]
pub struct WordleEngine {
    pub word_size: usize,
    pub word: String,
    pub guesses: Vec<Vec<LetterSlot>>,
    pub max_guess_count: usize,
    pub state: EngineState,
    pub current_guess: Vec<char>,
}

impl WordleEngine {
    pub fn new(word_size: usize) -> Self {
        let word = match word_size {
            4 => FOUR.choose(&mut rand::thread_rng()),
            5 => FIVE.choose(&mut rand::thread_rng()),
            6 => SIX.choose(&mut rand::thread_rng()),
            7 => SEVEN.choose(&mut rand::thread_rng()),
            _ => panic!("Invalid word size: {word_size}"),
        }
        .expect("Word not found")
        .to_string();
        WordleEngine {
            word_size,
            word,
            guesses: vec![],
            max_guess_count: word_size + 1,
            state: EngineState::Guessing,
            current_guess: vec![],
        }
    }
}

impl WordleEngine {
    pub fn add_letter(&mut self, letter: char) {
        if !letter.is_uppercase() {
            panic!("Uppercase only")
        }
        if self.state == EngineState::Guessing && self.current_guess.len() < self.word_size {
            self.current_guess.push(letter);
        }
    }

    pub fn submit(&mut self) -> Result<Option<SubmittedGuessInfo>, &'static str> {
        if self.state == EngineState::Guessing && self.current_guess.len() == self.word_size {
            if !self.is_word(self.current_guess.iter().collect()) {
                return Err("Not a word");
            }

            let mut output = SubmittedGuessInfo {
                word: self.current_guess.iter().collect(),
                ..SubmittedGuessInfo::default()
            };
            let row: Vec<LetterSlot> = self
                .current_guess
                .iter()
                .enumerate()
                .map(|(i, chr)| {
                    if self.word.chars().nth(i).unwrap() == *chr {
                        output.matches.push(*chr);
                        LetterSlot::new(*chr, SlotState::Match)
                    } else if self.word.contains(*chr) {
                        output.mismatches.push(*chr);
                        LetterSlot::new(*chr, SlotState::WrongPos)
                    } else {
                        output.no_matches.push(*chr);
                        LetterSlot::new(*chr, SlotState::NoMatch)
                    }
                })
                .collect();
            let word_found = row.iter().all(|slot| slot.state == SlotState::Match);
            self.guesses.push(row);
            if word_found {
                self.state = EngineState::Found
            } else if self.guesses.len() >= self.max_guess_count {
                self.state = EngineState::OutOfGuesses
            }
            self.current_guess.clear();
            return Ok(Some(output));
        }
        Ok(None)
    }

    pub fn backspace(&mut self) {
        if self.state == EngineState::Guessing && !self.current_guess.is_empty() {
            self.current_guess.remove(self.current_guess.len() - 1);
        }
    }

    fn is_word(&self, word: String) -> bool {
        let list: &[&str] = match self.word_size {
            4 => &FOUR,
            5 => &FIVE,
            6 => &SIX,
            7 => &SEVEN,
            _ => panic!("Invalid word size: {}", self.word_size),
        };

        list.contains(&word.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_typing() {
        let mut engine = WordleEngine::new(4);
        engine.add_letter('A');
        assert_eq!(engine.state, EngineState::Guessing);
        assert_eq!(engine.current_guess, vec!['A']);
        engine.backspace();
        assert_eq!(engine.current_guess, vec![]);
    }

    #[test]
    fn basic_play() {
        let mut engine = WordleEngine::new(4);
        engine.word = "TORT".to_string();

        assert_eq!(engine.guesses, Vec::<Vec<LetterSlot>>::new());
        assert_eq!(engine.current_guess, vec![]);
        assert_eq!(engine.state, EngineState::Guessing);
        engine.add_letter('Q');
        assert_eq!(engine.current_guess, vec!['Q']);
        assert_eq!(engine.state, EngineState::Guessing);
        engine.add_letter('Q');
        assert_eq!(engine.current_guess, vec!['Q', 'Q']);
        assert_eq!(engine.state, EngineState::Guessing);
        engine.add_letter('A');
        assert_eq!(engine.current_guess, vec!['Q', 'Q', 'A']);
        assert_eq!(engine.state, EngineState::Guessing);
        engine.add_letter('S');
        assert_eq!(engine.current_guess, vec!['Q', 'Q', 'A', 'S']);
        assert_eq!(engine.state, EngineState::Guessing);
        assert_eq!(engine.guesses, Vec::<Vec<LetterSlot>>::new());
        assert!(engine.submit().is_err());
        assert_eq!(engine.guesses, Vec::<Vec<LetterSlot>>::new());
        engine.current_guess = vec!['O', 'V', 'E', 'R'];
        assert!(engine.submit().is_ok());
        assert_eq!(
            engine.guesses,
            vec![vec![
                LetterSlot::new('O', SlotState::WrongPos),
                LetterSlot::new('V', SlotState::NoMatch),
                LetterSlot::new('E', SlotState::NoMatch),
                LetterSlot::new('R', SlotState::WrongPos),
            ]]
        );
        assert_eq!(engine.current_guess, vec![]);
        assert_eq!(engine.state, EngineState::Guessing);
        engine.add_letter('T');
        engine.add_letter('O');
        engine.add_letter('R');
        engine.add_letter('T');
        assert!(engine.submit().is_ok());
        assert_eq!(engine.state, EngineState::Found);
    }
}
