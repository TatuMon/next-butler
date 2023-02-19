use std::{path::PathBuf, fs, error::Error};

use crate::{helpers::{file_helper, str_helper}, commands::CommandError};

pub fn create(target_folder: &mut PathBuf, file_path: &mut String) -> Result<(), Box<dyn Error>> {
    target_folder.push("pages");
    fs::create_dir_all(&target_folder)?;
    file_path.push_str(".tsx");

    let final_path = target_folder.join(&file_path);

    if let Some(name_osstr) = final_path.file_stem() {
        if let Some(name_str) = name_osstr.to_str() {
            let name_str = str_helper::str_to_pascal_case(name_str)?;
            file_helper::create(&final_path, page_template_string(&name_str).as_bytes())?;
        } else {
            return Err(Box::new(CommandError{ message: String::from("Wrong file name") }));
        }
    } else {
        return Err(Box::new(CommandError{ message: String::from("Wrong file name") }));
    }

    Ok(())
}

fn page_template_string(page_name: &str) -> String {
    format!("\
export default function {}(){{
    // Your code goes here
}}", page_name)
}