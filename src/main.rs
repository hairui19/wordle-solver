use wordle_solver::{Guess, MatchResult, Solver}; 

fn main() {
    use MatchResult::*; 
    let guess = Guess::new(['w', 'e', 'a', 'r', 'y'], [Correct, Wrong, Misplaced, Wrong, Wrong]); 
    println!("the guess is: {:?}", guess);
    let mut solver = Solver::new(); 

    solver.learn(&guess); 

    println!("solver result: {:?}", solver);
}
