use std::{
    env, fs,
    path::{Path, PathBuf}
};

/// A helper function for recursively copying a directory.
fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let entry_path = path.unwrap().path();
        let to = to.clone().join(entry_path.file_name().unwrap());

        if entry_path.is_file() {
            fs::copy(&entry_path, to.clone()).unwrap();
        } else if entry_path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&entry_path, to);
        } else { /* Skip other content */ }
    }
}

fn main() {
    let mut out_dir = env::var("OUT_DIR").unwrap();
    out_dir.push_str("/templates/");

    let target_dir = PathBuf::from(out_dir);
    let templates_dir = PathBuf::from("templates/");

    let _ = fs::create_dir(target_dir.clone());
    copy_dir(templates_dir, target_dir);
}
