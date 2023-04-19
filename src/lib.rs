use itertools::iproduct;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

mod solver;
pub use solver::Solver;

pub struct Wordle;

impl Wordle {
    fn check_guess(answer: &str, guess: &str) -> Guess {
        assert!(answer.len() == 5);
        assert!(guess.len() == 5);
        use self::MatchResult::*;

        let mut match_result = answer.chars().zip(guess.chars()).enumerate().fold(
            [MatchResult::default(); 5],
            |mut match_result, (i, (a, g))| {
                if a == g {
                    match_result[i] = Correct;
                } else {
                    if let Some(position) = guess.chars().enumerate().position(|(i, c)| {
                        match_result[i] == Wrong
                            || match_result[i] == NotEvaluated
                                && c != answer.chars().nth(i).unwrap()
                                && c == a
                    }) {
                        match_result[position] = Misplaced;
                    }
                }
                match_result
            },
        );

        let match_result: [MatchResult; 5] = match_result
            .into_iter()
            .map(|r| match r {
                NotEvaluated | Wrong => Wrong,
                Correct => Correct,
                Misplaced => Misplaced,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Guess {
            guess: guess.chars().collect::<Vec<_>>().try_into().unwrap(),
            match_result: match_result,
        }
    }
}

#[derive(Default, Debug)]
pub struct Guess {
    guess: [char; 5],
    match_result: [MatchResult; 5],
}

impl Guess {
    pub fn new(guess: [char; 5], match_result: [MatchResult; 5]) -> Self {
        Guess {
            guess,
            match_result,
        }
    }
    pub fn get_word(&self) -> String {
        self.guess.iter().collect::<String>()
    }

    pub fn get_result(&self) -> &[MatchResult; 5] {
        &self.match_result
    }
}

impl std::fmt::Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{}-{:?}", self.get_word(), self.get_result())
        )
    }
}

#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub enum MatchResult {
    #[default]
    NotEvaluated,
    Correct,
    Misplaced,
    Wrong,
}

impl MatchResult {
    pub fn get_cartesian_product() -> impl Iterator<Item = [MatchResult; 5]> {
        use MatchResult::*;
        iproduct!(
            [Correct, Misplaced, Wrong],
            [Correct, Misplaced, Wrong],
            [Correct, Misplaced, Wrong],
            [Correct, Misplaced, Wrong],
            [Correct, Misplaced, Wrong]
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
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
