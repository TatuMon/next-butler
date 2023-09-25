use std::{
    env,
    fmt::{self, Display, Formatter},
    fs,
    path::PathBuf,
};

pub const FORBIDDEN_FILENAME_CHARS: [char; 9] = ['/', '\\', ':', '*', '?', '\"', '<', '>', '|'];

pub fn create(path: &PathBuf, content: Vec<u8>) -> Result<(), String> {
    println!("Creating file in: {}", path.display());

    if let Some(parents) = path.parent() {
        if let Err(_) = fs::create_dir_all(parents) {
            return Err(String::from("Couldn't create parent folders"));
        }
    }

    if path.exists() {
        return Err(String::from("File already exists"));
    }

    if let Err(_) = fs::write(path, content) {
        return Err(String::from("Coudln't create the file"));
    }

    Ok(())
}

/// Defines if the working directory has a src/ folder or not.
pub fn is_src_present() -> Result<bool, String> {
    match env::current_dir() {
        Ok(mut working_dir) => {
            working_dir.push("src/");
            Ok(working_dir.exists())
        }
        Err(_) => Err(String::from(
            "There was an error finding the
                                   src directory",
        )),
    }
}

pub fn get_name_or_err(path: &PathBuf) -> Result<&str, String> {
    let file_name = path.file_name();

    match file_name {
        Some(name) => {
            if let Some(name_str) = name.to_str() {
                Ok(name_str)
            } else {
                Err(String::from("Invalid file name"))
            }
        }
        None => Err(String::from("Couldn't get the file name")),
    }
}

#[derive(Debug)]
pub struct FileHelperError {
    message: String,
}

impl std::error::Error for FileHelperError {}

impl FileHelperError {
    pub fn new(message: String) -> FileHelperError {
        FileHelperError { message }
    }
}

impl Display for FileHelperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}
