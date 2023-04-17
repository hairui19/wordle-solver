use wordle_solver::algorithms::Naive; 
use wordle_solver::Guess; 
use wordle_solver::LetterState;

fn main() {
    let mut naive = Naive::new(); 
    let mut guess = Guess::default(); 
    // println!("before processing to result:{:?}", naive); 
    guess.letter_stats[0].state = LetterState::Wrong; 
    guess.letter_stats[0].guessed_letter = 'g'; 
    guess.letter_stats[1].state = LetterState::Wrong; 
    guess.letter_stats[1].guessed_letter = 'a'; 
    guess.letter_stats[2].state = LetterState::Misplaced; 
    guess.letter_stats[2].guessed_letter = 'm'; 
    guess.letter_stats[3].state = LetterState::Misplaced; 
    guess.letter_stats[3].guessed_letter = 'e'; 
    guess.letter_stats[4].state = LetterState::Misplaced; 
    guess.letter_stats[4].guessed_letter = 'r'; 
    println!("the guess is: {:?}", guess); 
    println!("the guess is: {:?}", guess.get_word()); 
    naive.learn(&guess);
    println!("the end result:{:?}", naive); 
}
