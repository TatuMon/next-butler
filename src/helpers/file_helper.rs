use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;

pub const FORBIDDEN_FILENAME_CHARS: [char; 9] = ['/', '\\', ':', '*', '?', '\"', '<', '>', '|'];

pub fn create(path: &PathBuf, content: Vec<u8>) -> Result<(), String> {
    println!("Creating file in: {}", path.display());

    let parents = path
        .parent()
        .ok_or(String::from("Couldn't get parent directory"))?;
    if fs::create_dir_all(parents).is_err() {
        return Err(String::from("Couldn't create parent folders"));
    }

    let filestem = path
        .file_stem()
        .ok_or(format!("{} must be a file", path.display()))?;

    if path.exists() || !get_file_stem_occurrences(filestem, parents)?.is_empty() {
        return Err(format!(
            "{} already exists but with a different extension",
            path.display()
        ));
    }

    if fs::write(path, content).is_err() {
        return Err(format!("Coudln't create {}", path.display()));
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

/// Gets all the occurrences of a file stem (the name without the extension)
///
/// # Arguments
///
/// * `file_stem` - The name of the file, without the extension
/// * `dir` - The directory where to look at
///
/// # Returns
///
/// A vector holding the paths of the founded files
pub fn get_file_stem_occurrences(
    file_stem: &OsStr,
    dir: impl AsRef<Path>,
) -> Result<Vec<PathBuf>, String> {
    let mut file_occurrences: Vec<PathBuf> = vec![];
    if !dir.as_ref().is_dir() {
        return Err(String::from("The provided path is not a directory"));
    }

    for dir_iter in fs::read_dir(dir.as_ref()).map_err(|err| err.to_string())? {
        match dir_iter {
            Ok(dir_entry) => {
                let dir_entry_path = dir_entry.path();
                if !dir_entry_path.is_file() {
                    continue;
                }

                if let Some(entry_stem) = dir_entry_path.file_stem() {
                    if entry_stem == file_stem {
                        file_occurrences.push(dir_entry_path);
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }

    Ok(file_occurrences)
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

pub fn json_file_to_struct<P, T>(file: &P) -> Result<T, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file_hndl = File::open(file)?;
    let reader = BufReader::new(file_hndl);

    let data = serde_json::from_reader(reader)?;

    Ok(data)
}
