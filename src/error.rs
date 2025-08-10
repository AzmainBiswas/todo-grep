use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct TgError {
    details: String,
}

impl fmt::Display for TgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for TgError {}

impl TgError {
    pub fn new(details: &str) -> Self {
        Self {
            details: details.into(),
        }
    }
}
