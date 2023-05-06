use path_clean::clean;
use regex::Regex;
use std::{
    env,
    fmt::{self, Display, Formatter},
    fs,
    path::PathBuf,
};

pub const FORBIDDEN_FILENAME_CHARS: [char; 9] = ['/', '\\', ':', '*', '?', '\"', '<', '>', '|'];

pub fn create(path: &PathBuf, content: &Vec<u8>) -> Result<(), String> {
    if let Some(parents) = path.parent() {
        if let Err(_) = fs::create_dir_all(parents) {
            return Err(String::from("Couldn't create parent folders"));
        }
    }

    println!("{}", path.to_string_lossy());

    if path.exists() {
        return Err(String::from("File already exists"));
    }

    if let Err(_) = fs::write(path, content) {
        return Err(String::from("Coudln't create the file"));
    }

    Ok(())
}

/// # Arguments
///
/// * `path` - It should be the value returned from ArgMatches::get_one
/// (or similar)
pub fn validate_filepath(path_str: &String) -> Result<PathBuf, String> {
    let path = clean(path_str.trim_matches(|c| c == '/' || c == '\\'));
    for comp in path.iter() {
        return match comp.to_str() {
            Some(str_comp) => {
                let re = Regex::new(r"^[a-zA-Z0-9_.-]+$").unwrap();
                if !re.is_match(str_comp) {
                    return Err(String::from("Invalid filepath"));
                }

                Ok(path)
            }
            None => Err(String::from("Invalid filepath")),
        };
    }

    return Ok(path);
}

/// Defines if the working directory has a src/ folder or not.
pub fn is_src_present() -> Result<bool, String> {
    match env::current_dir() {
        Ok(mut working_dir) => {
            working_dir.push("src/");
            Ok(working_dir.exists())
        },
        Err(_) => Err(String::from("There was an error finding the
                                   src directory"))
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
