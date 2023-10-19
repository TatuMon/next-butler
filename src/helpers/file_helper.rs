use std::{
    env,
    fmt::{self, Display, Formatter},
    fs,
    path::{Path, PathBuf}, ffi::OsStr,
};

pub const FORBIDDEN_FILENAME_CHARS: [char; 9] = ['/', '\\', ':', '*', '?', '\"', '<', '>', '|'];

pub fn create(path: &PathBuf, content: Vec<u8>) -> Result<(), String> {
    println!("Creating file in: {}", path.display());

    if let Some(parents) = path.parent() {
        if fs::create_dir_all(parents).is_err() {
            return Err(String::from("Couldn't create parent folders"));
        }
    }

    if path.exists() {
        return Err(String::from("File already exists"));
    }

    if fs::write(path, content).is_err() {
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

pub fn get_name_or_err(path: &Path) -> Result<&str, String> {
    let file_name = path.file_stem();

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

pub fn eq_file_name<P, T>(path1: &P, path2: &T) -> bool
where
    P: AsRef<Path>,
    T: AsRef<Path>,
{
    let path_buf_1: PathBuf = path1.as_ref().into();
    let path_buf_2: PathBuf = path2.as_ref().into();

    if let Some(stem_1) = path_buf_1.file_stem() {
        if let Some(stem_2) = path_buf_2.file_stem() {
            stem_1.to_string_lossy() == stem_2.to_string_lossy()
        } else {
            false
        }
    } else {
        false
    }
}

pub fn eq_file_extensions(ext1: Option<&OsStr>, ext2: Option<&OsStr>) -> bool {
    match ext1 {
        Some(ext1) => {
            if let Some(ext2) = ext2 {
                ext2.to_string_lossy() == ext1.to_string_lossy()
            } else {
                false
            }
        }
        None => ext1 == ext2,
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
