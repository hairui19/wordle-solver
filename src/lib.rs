use crate::error::InvalidInput;
use std::ops::{Index, IndexMut};

mod algorithms;
mod error;

pub struct Wordle;

impl Wordle {
    fn check_guess(answer: &str, guess: &str) -> Result<GuessResult, InvalidInput> {
        if answer.len() != 5 {
            return Err(InvalidInput::new(format!(
                "answer:{} does not contain exactly 5 letters",
                answer
            )));
        }
        if guess.len() != 5 {
            return Err(InvalidInput::new(format!(
                "guess:{} does not contain exactly 5 letters",
                guess
            )));
        }

        Ok(answer.chars().zip(guess.chars()).enumerate().fold(
            GuessResult::default(),
            |mut game_result, (i, (a, g))| {
                use self::LetterState::*;
                if a == g {
                    game_result[i] = Correct;
                } else {
                    if let Some(position) = guess
                        .chars()
                        .enumerate()
                        .position(|(i, c)| game_result[i] == Wrong && c == a)
                    {
                        game_result[position] = Misplaced;
                    }
                }
                game_result
            },
        ))
    }
}

#[derive(Default, PartialEq)]
pub enum LetterState {
    #[default]
    Wrong,
    Misplaced,
    Correct,
}

#[derive(Default)]
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
