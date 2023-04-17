mod invalid_input; 
pub use invalid_input::InvalidInput; 

// We wanna prevent other use cases to create the error enum directly. 
pub struct Error {
    inner: ErrorKind, 
}

enum ErrorKind {
    Input(InvalidInput), 
}