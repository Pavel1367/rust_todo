#[derive(Debug)]
pub struct ErrorMessage {
    pub message: String,
    pub created_at: std::time::Instant,
}

impl ErrorMessage {
    pub fn new(message: String) -> Self {
        ErrorMessage {
            message,
            created_at: std::time::Instant::now(),
        }
    }
}