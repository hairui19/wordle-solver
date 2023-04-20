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

impl std::str::FromStr for Guess {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once("-").unwrap();
        let word = split.0;
        let result = split.1;
        Ok(Guess {
            guess: word.chars().collect::<Vec<_>>().try_into().unwrap(),
            match_result: result
                .bytes()
                .map(|b| match b {
                    b'c' => MatchResult::Correct,
                    b'w' => MatchResult::Wrong,
                    b'm' => MatchResult::Misplaced,
                    _ => MatchResult::NotEvaluated,
                })
                .collect::<Vec<_>>()
                .try_into().unwrap(),
        })
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
