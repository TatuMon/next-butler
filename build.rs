use std::{path::{Path, PathBuf}, fs, env, process::Output};

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

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let templates_dir = PathBuf::from("templates");
    let target_dir = PathBuf::from(out_dir);

    copy_dir(templates_dir, target_dir);
}

