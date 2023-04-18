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

    // pub fn learn(&mut self, failed_guess: &Guess) {
    //     self.remaining_words
    //         .remove(failed_guess.get_word().as_str());
    //     self.remaining_words.retain(|word| {
    //         let mut used = [false; 5];
    //         for (index, letter_stat) in failed_guess.letter_stats.iter().enumerate() {
    //             if letter_stat.state == LetterState::Correct {
    //                 if word.chars().nth(index).unwrap() != letter_stat.guessed_letter {
    //                     return false;
    //                 } else {
    //                     used[index] = true;
    //                 }
    //             }
    //         }

    //         // println!("used stats before misplaced processing: {:?}", used);
    //         for (index, letter_stat) in failed_guess.letter_stats.iter().enumerate() {
    //             // moire
    //             if letter_stat.state == LetterState::Misplaced {
    //                 // println!("used stats: {:?}", used);
    //                 // println!("the letter tha twe are checking: {:?}", letter_stat);
    //                 if let Some((index, _)) = word
    //                     .chars()
    //                     .enumerate()
    //                     .filter(|(i, _)| !used[*i])
    //                     .find(|(_, c)| *c == letter_stat.guessed_letter)
    //                 {
    //                     used[index] = true;
    //                 } else {
    //                     // println!("used stats before return: {:?}", used);
    //                     return false;
    //                 }
    //             }
    //         }

    //         for (index, letter_stat) in failed_guess.letter_stats.iter().enumerate() {
    //             if letter_stat.state == LetterState::Wrong {
    //                 if let Some(_) = word
    //                     .chars()
    //                     .enumerate()
    //                     .filter(|(i, _)| !used[*i])
    //                     .position(|(_, c)| c == letter_stat.guessed_letter)
    //                 {
    //                     return false;
    //                 }
    //             }
    //         }

    //         true
    //     })
    // }
}
