use core::fmt;
use std::fmt::{Debug, Display, Formatter};

pub const VALID_SEPARATORS: [char; 3] = ['.', '-', '_'];

pub fn str_to_pascal_case(from: &str) -> Result<String, StrHelperError> {
    let mut parsed_str = String::new();
    let mut found_modifier = false;

    for (i, ch) in from.char_indices() {
        if !VALID_SEPARATORS.contains(&ch) && !ch.is_alphanumeric() {
            return Err(StrHelperError::new(String::from(format!("Wrong string character: {}", ch))));
        }

        if i == 0 {
            if !ch.is_alphabetic() {
                return Err(StrHelperError::new(String::from("First character must be alphabetic")));
            }
            parsed_str.push(ch.to_ascii_uppercase());
        } else if i > 0 {
            if VALID_SEPARATORS.contains(&ch) {
                found_modifier = true;
            } else if ch.is_alphanumeric() {
                if found_modifier {
                    found_modifier = false;
                    parsed_str.push(ch.to_ascii_uppercase());
                } else {
                    parsed_str.push(ch);
                }
            }
        }
    }

    Ok(parsed_str)
}

#[derive(Debug)]
pub struct StrHelperError {
    message: String
}

impl std::error::Error for StrHelperError {}

impl StrHelperError {
    pub fn new(message: String) -> StrHelperError {
        StrHelperError { message: message }
    }
}

impl Display for StrHelperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[STR HELPER ERROR] {}", &self.message)
    }
}