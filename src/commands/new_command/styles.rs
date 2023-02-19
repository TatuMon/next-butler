use std::{fs, path::PathBuf, error::Error};

use crate::{helpers::file_helper, commands::CommandError};

pub fn create(target_folder: &mut PathBuf, file_path: &mut String) -> Result<(), Box<dyn Error>> {
    target_folder.push("styles");
    fs::create_dir_all(&target_folder)?;
    file_path.push_str(".scss");

    let final_path = target_folder.join(&file_path);

    if let Some(_) = final_path.file_stem() {
        file_helper::create(&final_path, style_template_string().as_bytes())?;
    } else {
        return Err(Box::new(CommandError{ message: String::from("Wrong file name") }));
    }

    Ok(())
}

fn style_template_string() -> String {
    format!("\
// I can't think of a template :( Should it have?")
}