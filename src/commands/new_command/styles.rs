use std::{fs, path::PathBuf, error::Error};

use crate::{helpers::file_helper, commands::CommandError, json_config::new_style_config::NewStyleConfig};

pub const STYLES_DEFAULT_FOLDER: &str = "styles";

pub fn create(target_folder: &mut PathBuf, file_path: &mut String) -> Result<(), Box<dyn Error>> {
    target_folder.push(STYLES_DEFAULT_FOLDER);
    fs::create_dir_all(&target_folder)?;

    match NewStyleConfig::build() {
        Ok(new_style_config) => {
            if !new_style_config.ext.is_empty() {
                file_path.push_str(&new_style_config.ext);
            } else {
                file_path.push_str(".css");
            }
        },
        Err(_) => {
            file_path.push_str(".css");
        }
    }

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
/* I can't think of a template :( Should it have? */")
}