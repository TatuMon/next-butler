use core::fmt;
use std::{path::PathBuf, fs, error::Error, fmt::{Formatter, Display}};

pub fn create(path: &PathBuf, content: &[u8]) -> Result<(), Box<dyn Error>> {
    if let Some(parents) = path.parent() {
        fs::create_dir_all(parents)?;
    }

    if path.exists() {
        return Err(Box::new(FileHelperError::new(String::from("File already exists"))));
    }

    fs::write(path, content)?;

    Ok(())
}

#[derive(Debug)]
pub struct FileHelperError {
    message: String
}

impl std::error::Error for FileHelperError {}

impl FileHelperError {
    pub fn new(message: String) -> FileHelperError {
        FileHelperError { message: message }
    }
}

impl Display for FileHelperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}