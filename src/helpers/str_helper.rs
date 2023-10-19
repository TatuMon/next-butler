use core::fmt;
use std::fmt::{Debug, Display, Formatter};

pub const VALID_SEPARATORS: [char; 3] = ['.', '-', '_'];

/// Split the slice by the last occurrence of the given char
/// and returns a tuple that holds the two resulting slices
pub fn split_last(word: &str, ch: char) -> Option<(&str, &str)> {
    word.rfind(ch).map(|ch_pos| word.split_at(ch_pos))
}

#[derive(Debug)]
pub struct StrHelperError {
    message: String,
}

impl std::error::Error for StrHelperError {}

impl StrHelperError {
    pub fn new(message: String) -> StrHelperError {
        StrHelperError { message }
    }
}

impl Display for StrHelperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}
