use wordle_solver::algorithms::Naive; 
use wordle_solver::Guess; 
use wordle_solver::LetterState;

fn main() {
    let mut naive = Naive::new(); 
    let mut guess_1 = Guess::default(); 
    // println!("before processing to result:{:?}", naive); 
    guess_1.letter_stats[0].state = LetterState::Wrong; 
    guess_1.letter_stats[0].guessed_letter = 'w'; 
    guess_1.letter_stats[1].state = LetterState::Correct; 
    guess_1.letter_stats[1].guessed_letter = 'i'; 
    guess_1.letter_stats[2].state = LetterState::Wrong; 
    guess_1.letter_stats[2].guessed_letter = 't'; 
    guess_1.letter_stats[3].state = LetterState::Correct; 
    guess_1.letter_stats[3].guessed_letter = 'c'; 
    guess_1.letter_stats[4].state = LetterState::Wrong; 
    guess_1.letter_stats[4].guessed_letter = 'h'; 

    let mut guess_2 = Guess::default(); 
    // println!("before processing to result:{:?}", naive); 
    guess_2.letter_stats[0].state = LetterState::Correct; 
    guess_2.letter_stats[0].guessed_letter = 'n'; 
    guess_2.letter_stats[1].state = LetterState::Wrong; 
    guess_2.letter_stats[1].guessed_letter = 'p'; 
    guess_2.letter_stats[2].state = LetterState::Misplaced; 
    guess_2.letter_stats[2].guessed_letter = 'e'; 
    guess_2.letter_stats[3].state = LetterState::Wrong; 
    guess_2.letter_stats[3].guessed_letter = 'n'; 
    guess_2.letter_stats[4].state = LetterState::Wrong; 
    guess_2.letter_stats[4].guessed_letter = 'd'; 

    let mut guess_3 = Guess::default(); 
    // println!("before processing to result:{:?}", naive); 
    guess_3.letter_stats[0].state = LetterState::Correct; 
    guess_3.letter_stats[0].guessed_letter = 's'; 
    guess_3.letter_stats[1].state = LetterState::Wrong; 
    guess_3.letter_stats[1].guessed_letter = 'h'; 
    guess_3.letter_stats[2].state = LetterState::Misplaced; 
    guess_3.letter_stats[2].guessed_letter = 'e'; 
    guess_3.letter_stats[3].state = LetterState::Misplaced; 
    guess_3.letter_stats[3].guessed_letter = 'l'; 
    guess_3.letter_stats[4].state = LetterState::Wrong; 
    guess_3.letter_stats[4].guessed_letter = 'l'; 

    let mut guess_4 = Guess::default(); 
    // println!("before processing to result:{:?}", naive); 
    guess_4.letter_stats[0].state = LetterState::Correct; 
    guess_4.letter_stats[0].guessed_letter = 's'; 
    guess_4.letter_stats[1].state = LetterState::Misplaced; 
    guess_4.letter_stats[1].guessed_letter = 'l'; 
    guess_4.letter_stats[2].state = LetterState::Wrong; 
    guess_4.letter_stats[2].guessed_letter = 'i'; 
    guess_4.letter_stats[3].state = LetterState::Wrong; 
    guess_4.letter_stats[3].guessed_letter = 'c'; 
    guess_4.letter_stats[4].state = LetterState::Correct; 
    guess_4.letter_stats[4].guessed_letter = 'e';

    let mut guess_5 = Guess::default(); 
    // println!("before processing to result:{:?}", naive); 
    guess_5.letter_stats[0].state = LetterState::Correct; 
    guess_5.letter_stats[0].guessed_letter = 's'; 
    guess_5.letter_stats[1].state = LetterState::Wrong; 
    guess_5.letter_stats[1].guessed_letter = 't'; 
    guess_5.letter_stats[2].state = LetterState::Wrong; 
    guess_5.letter_stats[2].guessed_letter = 'y'; 
    guess_5.letter_stats[3].state = LetterState::Misplaced; 
    guess_5.letter_stats[3].guessed_letter = 'l'; 
    guess_5.letter_stats[4].state = LetterState::Correct; 
    guess_5.letter_stats[4].guessed_letter = 'e';

    naive.learn_batch(&[guess_1]);
    println!("the end result:{:?}", naive); 
}
