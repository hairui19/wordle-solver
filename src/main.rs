use itertools::iproduct;
use std::collections::HashMap;
use std::fmt::Display;
use wordle_solver::{Guess, MatchResult, Solver};

fn main() {
    use MatchResult::*;
    let guess = Guess::new(
        ['s', 'l', 'a', 't', 'e'],
        [Wrong, Misplaced, Wrong, Wrong, Wrong],
    );

    let dictionary: &str = include_str!("../dictionary.txt");
    let mut hash_map = HashMap::<String, f64>::new();
    for word in dictionary
        .lines()
        .map(|line| line.split_once(" ").unwrap().0)
    {
        println!("Processing word: {}", word);
        let mut sum: f64 = 0.0;
        for match_combination in MatchResult::get_cartesian_product() {
            let guess = Guess::new(
                word.chars().collect::<Vec<char>>().try_into().unwrap(),
                match_combination,
            );
            let mut solver = Solver::new();
            if let Some(average_bits_info) = solver.calculate_average_bits_info(&guess) {
                sum += average_bits_info;
            }
        }
        hash_map.entry(word.to_string()).or_insert(sum);
    }

    let mut vec = hash_map.into_iter().collect::<Vec<_>>();
    vec.sort_by(|(_, p_1), (_, p_2)| p_1.partial_cmp(p_2).unwrap_or(std::cmp::Ordering::Equal));

    println!("{:?}", vec);
}
