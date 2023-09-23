use std::{
    env, fs,
    path::{Path, PathBuf},
};

const COPY_DIR: &'static str = "templates";

/// A helper function for recursively copying a directory.
fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        } else { /* Skip other content */
        }
    }
}

use std::fs;

fn main() {
    // Path to the "templates" folder
    let source_dir = "templates";

    // Path to the target directory where you want to include "templates"
    let target_dir = concat!(env!("OUT_DIR"), "/templates");

    // Create the target directory if it doesn't exist
    fs::create_dir_all(target_dir).unwrap();

    // Copy the contents of the "templates" folder to the target directory
    fs::copy_dir(source_dir, target_dir).unwrap();
}

