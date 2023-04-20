use crate::Guess;
use crate::MatchResult;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;

const FIRST_GUESSES_FILE_PATH: &str = "first_guesses_file.json";

pub struct Solver {
    remaining_words: Vec<&'static str>,
    match_combinations: Vec<[MatchResult; 5]>,
    is_first_guess: bool,
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
            match_combinations: MatchResult::get_cartesian_product().collect::<Vec<_>>(),
            is_first_guess: true,
        }
    }

    pub fn suggest_top_ten_words(&self) -> Vec<(String, f64)> {
        if self.is_first_guess {
            let file = File::open(FIRST_GUESSES_FILE_PATH).unwrap();
            let entropies_map: HashMap<String, f64> = serde_json::from_reader(file).unwrap();
            let mut vec = entropies_map.into_iter().collect::<Vec<_>>();
            vec.sort_by(|(_, p_1), (_, p_2)| {
                p_2.partial_cmp(p_1).unwrap_or(std::cmp::Ordering::Equal)
            });

            return vec.into_iter().take(10).collect();
        } else {
            let entropies_map = self.calculate_entropies_for_remaining_words();
            let mut vec = entropies_map.into_iter().collect::<Vec<_>>();
            vec.sort_by(|(_, p_1), (_, p_2)| {
                p_2.partial_cmp(p_1).unwrap_or(std::cmp::Ordering::Equal)
            });

            return vec
                .into_iter()
                .take(10)
                .map(|(word, entropy)| (word.to_string(), entropy))
                .collect();
        }
    }

    pub fn calibrate_on_guess(&mut self, guess: &Guess) {
        let guess_word = &guess.get_word();
        let guess_result = guess.get_result();
        self.remaining_words
            .retain(|word| Self::is_matching(word, guess_word, guess_result));
        self.is_first_guess = false; 
    }

    fn calibrate(&self) {
        let hash_map = self.calculate_entropies_for_remaining_words();
        let file = File::create(FIRST_GUESSES_FILE_PATH).unwrap();
        serde_json::to_writer_pretty(file, &hash_map);
    }

    fn calculate_entropies_for_remaining_words(&self) -> HashMap<&'static str, f64> {
        let mut entropies_map = HashMap::<&'static str, f64>::new();
        for word in self.remaining_words.iter() {
            let word_entropy = self.calculate_entropy_for_word(word);
            entropies_map.entry(word).or_insert(word_entropy);
        }
        return entropies_map;
    }

    /// Calculates the entropy value of the input `guess_word` against
    /// the remaining set of words in the solver.
    fn calculate_entropy_for_word(&self, guess_word: &str) -> f64 {
        // println!("Calibrating word: {}", guess_word);
        let mut entropy = 0.0;
        let total_remaining_word_count = self.remaining_words.len();
        for match_combination in self.match_combinations.iter() {
            let number_of_matching_words =
                self.get_number_of_matching_words(guess_word, match_combination);
            if number_of_matching_words == 0 {
                continue;
            }
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
