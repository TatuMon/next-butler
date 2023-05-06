use core::fmt;
use std::fmt::{Debug, Display, Formatter};

use crate::constants::COMPONENTS_DEFAULT_FOLDER;

use super::file_helper::FORBIDDEN_FILENAME_CHARS;

pub const VALID_SEPARATORS: [char; 3] = ['.', '-', '_'];

pub fn str_to_pascal_case(from: &str) -> Result<String, StrHelperError> {
    let mut parsed_str = String::new();
    let mut use_upper = true;

    for ch in from.chars() {
        if FORBIDDEN_FILENAME_CHARS.contains(&ch) {
            return Err(StrHelperError::new(String::from(format!(
                "Wrong string character: {}",
                ch
            ))));
        }

        if VALID_SEPARATORS.contains(&ch) {
            use_upper = true;
        } else if ch.is_alphanumeric() {
            if use_upper {
                parsed_str.push(ch.to_ascii_uppercase());
                use_upper = false;
            } else {
                parsed_str.push(ch.to_ascii_lowercase());
            }
        }
    }

    Ok(parsed_str)
}

/**
 * Add a dot at the beginning (if doesn't have one already)
 *
 * If from is empty, nothing changes
 */
pub fn to_ext(from: &str) -> String {
    if from.is_empty() {
        return String::from(from);
    }

    let mut parsed = String::new();

    for (i, ch) in from.char_indices() {
        if from.len() == 1 && !ch.is_alphabetic() {
            return parsed;
        } else if i == 0 && ch != '.' {
            parsed.push('.');
            parsed.push(ch);
        } else {
            parsed.push(ch);
        }
    }

    parsed
}

pub fn to_folder_name(from: &str) -> String {
    let mut parsed = String::new();

    for (i, ch) in from.char_indices() {
        if from.len() == 1 && !ch.is_ascii_alphabetic() {
            return String::from(COMPONENTS_DEFAULT_FOLDER);
        } else {
            if i == 0 && (ch.is_alphabetic() || ch == '.') {
                parsed.push(ch);
            } else {
                if !ch.is_alphabetic() && !VALID_SEPARATORS.contains(&ch) && i < from.len() - 1 {
                    return String::from(COMPONENTS_DEFAULT_FOLDER);
                } else if i == from.len() - 1 && ch.is_whitespace() {
                    break;
                } else {
                    parsed.push(ch);
                }
            }
        }
    }

    parsed
}

#[derive(Debug)]
pub struct StrHelperError {
    message: String,
}

impl std::error::Error for StrHelperError {}

impl StrHelperError {
    pub fn new(message: String) -> StrHelperError {
        StrHelperError { message: message }
    }
}

impl Display for StrHelperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}
