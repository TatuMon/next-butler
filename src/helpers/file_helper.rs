use std::{path::PathBuf, fs, error::Error};

pub fn create(path: &PathBuf, content: &[u8]) -> Result<(), Box<dyn Error>> {
    if let Some(parents) = path.parent() {
        fs::create_dir_all(parents)?;
    }

    fs::write(path, content)?;

    Ok(())
}