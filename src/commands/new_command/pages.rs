use std::{error::Error, fs, path::PathBuf};

use crate::{
    commands::command_error::CommandError,
    helpers::{file_helper, str_helper, template_helper::{get_page_content, get_api_page_content}},
    json_config::new_page_config::NewPageConfig,
};

use super::new_command_config::NewCommandConfig;

pub const PAGES_DEFAULT_FOLDER: &str = "pages";

pub fn create(
    command_config: &mut NewCommandConfig,
    options: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    process_pages_options(options);

    let target_folder = &mut command_config.target_folder;
    let file_path = &mut command_config.file_name;

    target_folder.push(PAGES_DEFAULT_FOLDER);
    fs::create_dir_all(&target_folder)?;

    let is_api = is_api_page(&file_path);
    match NewPageConfig::build() {
        Ok(new_page_config) => {
            if new_page_config.typescript {
                if !new_page_config.use_jsx || is_api {
                    file_path.push_str(".ts");
                } else {
                    file_path.push_str(".tsx");
                }
            } else {
                if !new_page_config.use_jsx || is_api {
                    file_path.push_str(".js");
                } else {
                    file_path.push_str(".jsx");
                }
            }
        }
        Err(_) => {
            file_path.push_str(".js");
        }
    }

    let final_path = target_folder.join(&file_path);

    write_page(&final_path, is_api)
}

fn write_page(final_path: &PathBuf, is_api: bool) -> Result<(), Box<dyn Error>> {
    // Tries to get the file without extension
    if let Some(name_osstr) = final_path.file_stem() {
        // Tries to get the name OS string as a normal String
        if let Some(name_str) = name_osstr.to_str() {
            let name_str = str_helper::str_to_pascal_case(name_str)?;

            if is_api {
                file_helper::create(&final_path, get_api_page_content()?.as_bytes())?;
            } else {
                file_helper::create(&final_path, get_page_content(&name_str)?.as_bytes())?;
            }
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

fn is_api_page(target_path: &String) -> bool {
    target_path.starts_with("api")
        || target_path.starts_with("/api")
        || target_path.starts_with("\\api")
}

fn process_pages_options(options: &Vec<String>) {
    for opt in options {
        match opt.as_str() {
            "--help" => show_page_help(),
            "--ts" => set_page_ext("ts"),
            "--tsx" => set_page_ext("tsx"),
            "--js" => set_page_ext("js"),
            "--jsx" => set_page_ext("jsx"),
            _ => (),
        }
    }
}

fn show_page_help() {
    println!(
        "Creates a new page with the given name. To know how you can name your pages, visit
https://nextjs.org/docs/basic-features/pages"
    );
}

fn set_page_ext(ext: &str) {}
