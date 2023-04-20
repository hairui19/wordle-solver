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
    // for word in dictionary
    //     .lines()
    //     .map(|line| line.split_once(" ").unwrap().0)
    let mut solver = Solver::new(all_words);
    for word in vec!["weary"].into_iter() {
        println!("Processing word: {}", word);
        hash_map.entry(word).or_insert(solver.calculate_entropy(word));
    }

    let mut vec = hash_map.into_iter().collect::<Vec<_>>();
    vec.sort_by(|(_, p_1), (_, p_2)| p_1.partial_cmp(p_2).unwrap_or(std::cmp::Ordering::Equal));

    println!("{:?}", vec);
}
