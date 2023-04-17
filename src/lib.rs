use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub mod algorithms;
mod error;

pub struct Wordle;

impl Wordle {
    fn check_guess(answer: &str, guess: &str) -> GuessResult {
        assert!(answer.len() == 5);
        assert!(guess.len() == 5);
        use self::LetterState::*;

        answer.chars().zip(guess.chars()).enumerate().fold(
            GuessResult::default(),
            |mut guess_result, (i, (a, g))| {
                if a == g {
                    guess_result[i] = Correct;
                } else {
                    if let Some(position) = guess.chars().enumerate().position(|(i, c)| {
                        guess_result[i] == Wrong && c != answer.chars().nth(i).unwrap() && c == a
                    }) {
                        guess_result[position] = Misplaced;
                    }
                }
                guess_result
            },
        )
    }
}

#[derive(Default, Debug)]
pub struct Guess {
    pub letter_stats: [LetterStat; 5],
}

impl Guess {
    pub fn new(guess: &str, guess_result: &GuessResult) -> Self {
        guess
            .chars()
            .zip(guess_result.letter_states.iter())
            .enumerate()
            .fold(Guess::default(), |mut guess, (i, (c, letter_state))| {
                guess.letter_stats[i] = LetterStat {
                    state: letter_state.clone(),
                    guessed_letter: c,
                };
                guess
            })
    }

    pub fn get_word(&self) -> String {
        self.letter_stats
            .iter()
            .map(|x| x.guessed_letter)
            .collect::<String>()
    }
}

#[derive(Default, Debug)]
pub struct LetterStat {
    pub state: LetterState,
    pub guessed_letter: char,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub enum LetterState {
    #[default]
    Wrong,
    Misplaced,
    Correct,
}

#[derive(Default, Debug)]
pub struct GuessResult {
    letter_states: [LetterState; 5],
}

impl Index<usize> for GuessResult {
    type Output = LetterState;

    fn index(&self, index: usize) -> &Self::Output {
        &self.letter_states[index]
    }
}

impl IndexMut<usize> for GuessResult {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.letter_states[index]
    }
}

#[cfg(test)]
impl PartialEq<[LetterState; 5]> for GuessResult {
    fn eq(&self, other: &[LetterState; 5]) -> bool {
        self.letter_states.len() == other.len()
            && self
                .letter_states
                .iter()
                .enumerate()
                .all(|(i, letter_state)| other[i] == *letter_state)
    }
}

#[cfg(test)]
impl PartialEq<GuessResult> for [LetterState; 5] {
    fn eq(&self, other: &GuessResult) -> bool {
        self.len() == other.letter_states.len()
            && self
                .iter()
                .enumerate()
                .all(|(i, letter_state)| other.letter_states[i] == *letter_state)
    }
}

#[cfg(test)]
mod tests {
    use crate::LetterState::*;
    use crate::Wordle;

    #[test]
    fn test_all_correct() {
        assert_eq!(
            Wordle::check_guess("guess", "guess"),
            [Correct, Correct, Correct, Correct, Correct]
        );
    }

    #[test]
    fn test_all_wrong() {
        assert_eq!(
            Wordle::check_guess("gamer", "books"),
            [Wrong, Wrong, Wrong, Wrong, Wrong]
        );
    }

    #[test]
    fn test_all_misplaced() {
        assert_eq!(
            Wordle::check_guess("books", "oncek"),
            [Misplaced, Wrong, Wrong, Wrong, Misplaced]
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Wordle::check_guess("azzaz", "aaabb"),
            [Correct, Misplaced, Wrong, Wrong, Wrong]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Wordle::check_guess("baccc", "aaddd"),
            [Wrong, Correct, Wrong, Wrong, Wrong]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Wordle::check_guess("abcde", "aacde"),
            [Correct, Wrong, Correct, Correct, Correct]
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Wordle::check_guess("acaca", "hhhch"),
            [Wrong, Wrong, Wrong, Correct, Wrong]
        );
    }

    fn test_5() {
        assert_eq!(
            Wordle::check_guess("acacc", "hhaha"),
            [Wrong, Wrong, Correct, Wrong, Misplaced]
        );
    }
}
