use std::{error::Error, fs};

use crate::{
    commands::command_error::CommandError,
    helpers::{file_helper, str_helper},
    json_config::new_component_config::NewComponentConfig,
};

use super::new_command_config::NewCommandConfig;

pub const COMPONENTS_DEFAULT_FOLDER: &str = "components";

pub fn create(
    command_config: &mut NewCommandConfig,
    options: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let target_folder = &mut command_config.target_folder;
    let file_path = &mut command_config.file_name;

    fs::create_dir_all(&target_folder)?;

    match NewComponentConfig::build() {
        Ok(new_page_config) => {
            if new_page_config.typescript {
                if new_page_config.use_jsx {
                    file_path.push_str(".tsx");
                } else {
                    file_path.push_str(".ts");
                }
            } else {
                if new_page_config.use_jsx {
                    file_path.push_str(".jsx");
                } else {
                    file_path.push_str(".js");
                }
            }

            if new_page_config.folder.is_empty() {
                target_folder.push(COMPONENTS_DEFAULT_FOLDER);
            } else {
                target_folder.push(new_page_config.folder);
            }
        }
        Err(_) => {
            file_path.push_str(".js");
            target_folder.push(COMPONENTS_DEFAULT_FOLDER);
        }
    }

    let final_path = target_folder.join(&file_path);

    if let Some(name_osstr) = final_path.file_stem() {
        if let Some(name_str) = name_osstr.to_str() {
            let name_str = str_helper::str_to_pascal_case(name_str)?;
            file_helper::create(&final_path, page_template_string(&name_str).as_bytes())?;
        } else {
            return Err(Box::new(CommandError {
                message: String::from("Wrong file name"),
            }));
        }
    } else {
        return Err(Box::new(CommandError {
            message: String::from("Wrong file name"),
        }));
    }

    Ok(())
}

fn page_template_string(page_name: &str) -> String {
    format!(
        "\
export default function {}(){{
    // Your code goes here
}}",
        page_name
    )
}
