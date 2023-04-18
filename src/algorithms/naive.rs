use crate::Guess;
use crate::LetterState;
use std::collections::HashSet;

const DICTIONARY: &str = include_str!("../../dictionary.txt");

#[derive(Debug)]
pub struct Naive {
    remaining_words: HashSet<&'static str>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining_words: HashSet::from_iter(
                DICTIONARY.lines().map(|str| str.split_once(" ").unwrap().0),
            ),
        }
    }

    pub fn learn_batch(&mut self, fail_guesses: &[Guess]) {
        for guess in fail_guesses {
            self.learn(guess);
        }
    }

    pub fn learn(&mut self, failed_guess: &Guess) {
        self.remaining_words
            .remove(failed_guess.get_word().as_str());
        self.remaining_words.retain(|word| {
            let mut used = [false; 5];
            for (index, letter_stat) in failed_guess.letter_stats.iter().enumerate() {
                if letter_stat.state == LetterState::Correct {
                    if word.chars().nth(index).unwrap() != letter_stat.guessed_letter {
                        return false;
                    } else {
                        used[index] = true;
                    }
                }
            }

            // println!("used stats before misplaced processing: {:?}", used);
            for (index, letter_stat) in failed_guess.letter_stats.iter().enumerate() {
                // moire
                if letter_stat.state == LetterState::Misplaced {
                    // println!("used stats: {:?}", used);
                    // println!("the letter tha twe are checking: {:?}", letter_stat);
                    if let Some((index, _)) = word
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| !used[*i])
                        .find(|(_, c)| *c == letter_stat.guessed_letter)
                    {
                        used[index] = true;
                    } else {
                        // println!("used stats before return: {:?}", used);
                        return false;
                    }
                }
            }

            for (index, letter_stat) in failed_guess.letter_stats.iter().enumerate() {
                if letter_stat.state == LetterState::Wrong {
                    if let Some(_) = word
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| !used[*i])
                        .position(|(_, c)| c == letter_stat.guessed_letter)
                    {
                        return false;
                    }
                }
            }

            true
        })
    }
}
