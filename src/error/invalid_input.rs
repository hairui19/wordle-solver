pub struct InvalidInput {
    error_message: String,
}

impl InvalidInput {
    pub fn new(error_message: String) -> Self {
        Self { error_message }
    }
}
