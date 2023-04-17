mod naive; 
pub use naive::Naive; 

trait Guesser {
    fn guess() -> String; 
}