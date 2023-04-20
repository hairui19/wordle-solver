use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use wordle_solver::{Guess, MatchResult, Solver};

fn main() {
    use MatchResult::*;
    let guess = Guess::new(
        ['s', 'l', 'a', 't', 'e'],
        [Wrong, Misplaced, Wrong, Wrong, Wrong],
    );

    let dictionary: &str = include_str!("../dictionary.txt");
    let all_words = Vec::from_iter(dictionary.lines().map(| line | line.split_once(" ").unwrap().0)); 
    let mut hash_map = HashMap::<&'static str, f64>::new();
    let mut solver = Solver::new(all_words);
    println!("{:?}", solver.suggest_top_ten_words()); 
}
