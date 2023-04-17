mod naive; 
use naive::Naive; 

trait Guesser {
    fn guess() -> String; 
}