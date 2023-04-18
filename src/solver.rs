use crate::Guess;
use std::collections::HashSet;
use std::fmt;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Solver {
    remaining_words: HashSet<&'static str>,
}

impl fmt::Debug for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{:?}-\n{}",
                self.remaining_words,
                self.remaining_words.len()
            )
        )
    }
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            remaining_words: HashSet::from_iter(
                DICTIONARY.lines().map(|str| str.split_once(" ").unwrap().0),
            ),
        }
    }

    pub fn learn(&mut self, guess: &Guess) {
        let guessed_word = guess.get_word();
        let guessed_result = guess.get_result();
        self.remaining_words.remove(guessed_word.as_str());
        use crate::MatchResult::*;
        self.remaining_words.retain(|word| {
            let mut used = [false; 5];

            for (i, ((w, g), r)) in word
                .chars()
                .zip(guessed_word.chars())
                .zip(guessed_result)
                .enumerate()
            {
                match r {
                    Correct => {
                        if w != g {
                            return false;
                        }
                        used[i] = true;
                    }
                    Wrong => {
                        if let Some(_) = word.chars().position(|c| c == g) {
                            return false;
                        }
                    }
                    Misplaced => {
                        if w == g {
                            return false;
                        }
                    }
                    NotEvaluated => unreachable!(),
                }
            }

            for (g_i, g) in guessed_word.chars().enumerate() {
                if guessed_result[g_i] != Misplaced {
                    continue;
                }
                if let Some((index, _)) = word
                    .chars()
                    .enumerate()
                    .filter(|(w_i, _)| !used[*w_i] && *w_i != g_i)
                    .find(|(_, w)| *w == g)
                {
                    used[index] = true
                } else {
                    return false;
                }
            }

            true
        })
    }

    pub fn calculate(&mut self, guess: &Guess) -> f64 {
        let total_count = self.remaining_words.len() as f64;
        self.learn(guess);
        let remaining_count = self.remaining_words.len() as f64;

        return remaining_count / total_count;
    }

    pub fn calculate_average_bits_info(&mut self, guess: &Guess) -> Option<f64> {
        let total_count = self.remaining_words.len() as f64;
        self.learn(guess);
        let remaining_count = self.remaining_words.len() as f64;
        let probability = remaining_count / total_count;
        let average_bits_info = 0.0 - probability.log2() * probability;
        if average_bits_info.is_infinite() || average_bits_info.is_nan() {
            None
        } else {
            Some(average_bits_info)
        }
    }
}