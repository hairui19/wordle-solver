use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Write;
use wordle_solver::{Guess, MatchResult, Solver};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn main() {
    use MatchResult::*;
    let guess = Guess::new(
        ['s', 'l', 'a', 't', 'e'],
        [Wrong, Misplaced, Wrong, Wrong, Wrong],
    );

    let dictionary: &str = include_str!("../dictionary_sigmoid.txt");
    let all_words = Vec::from_iter(
        dictionary
            .lines()
            .map(|line| line.split_once(" ").unwrap())
            .map(|(w, freq)| (w, freq.parse::<f64>().unwrap())),
    );
    let mut hash_map = HashMap::<&'static str, f64>::new();
    let mut solver = Solver::new(all_words);

    println!("{}", CLEAR);
    for suggested_word in solver.suggest_top_ten_words() {
        println!("{}-{}", suggested_word.0, suggested_word.1);
    }

    let mut buffer = String::new();
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let guess: Guess = buffer.trim().parse().unwrap();
        println!("Your guess is: {:?}", guess);
        solver.calibrate_on_guess(&guess);
        println!("{}", CLEAR);
        for suggested_word in solver.suggest_top_ten_words() {
            println!("{}-{}", suggested_word.0, suggested_word.1);
        }
    }
}

fn sigmoid(x: f64, a: f64) -> f64 {
    1.0 / (1.0 + (-a * x).exp())
}
