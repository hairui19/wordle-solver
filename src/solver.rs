use crate::Guess;
use crate::MatchResult;
use std::collections::HashSet;
use std::fmt;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Solver {
    remaining_words: Vec<&'static str>,
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
    pub fn new(words: Vec<&'static str>) -> Self {
        Solver {
            remaining_words: words,
        }
    }

    /// Calculates the entropy value of the input `guess_word` against
    /// the remaining set of words in the solver.
    pub fn calculate_entropy(&self, guess_word: &str) -> f64 {
        let mut entropy = 0.0; 
        let total_remaining_word_count = self.remaining_words.len();
        for match_combination in MatchResult::get_cartesian_product() {
            let number_of_matching_words = self.get_number_of_matching_words(guess_word, &match_combination); 
            if number_of_matching_words == 0 { continue; }
            let probability = number_of_matching_words as f64 / total_remaining_word_count as f64; 
            let average_bits_info = 0.0 - probability.log2() * probability;
            entropy += average_bits_info;
        }
        
        entropy
    }

    // Private function/methods.
    /// Get the total number of words that match the guess from the remaining set of words
    /// in the solver.
    fn get_number_of_matching_words(
        &self,
        guess_word: &str,
        match_combination: &[MatchResult; 5],
    ) -> usize {
        let mut number_of_matching_words = 0;

        for word in self.remaining_words.iter() {
            if Self::is_matching(word, guess_word, match_combination) {
                number_of_matching_words += 1;
            }
        }

        number_of_matching_words
    }

    /// Check the input word against the guess_word to see if they can be matched against the
    /// match_combination
    fn is_matching(word: &str, guess_word: &str, match_combination: &[MatchResult; 5]) -> bool {
        use crate::MatchResult::*;
        if word == guess_word {
            return false;
        }
        let mut used = [false; 5];

        for (i, ((w, g), r)) in word
            .as_bytes()
            .iter()
            .zip(guess_word.as_bytes().iter())
            .zip(match_combination)
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
                    if let Some(_) = word.as_bytes().iter().position(|c| c == g) {
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

        for (g_i, g) in guess_word.as_bytes().iter().enumerate() {
            if match_combination[g_i] != Misplaced {
                continue;
            }
            if let Some((index, _)) = word
                .as_bytes()
                .iter()
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
    }
}
