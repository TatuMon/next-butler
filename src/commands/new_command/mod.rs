use std::fs::{self};
use std::path::PathBuf;
use std::{error::Error};

use super::super::BaseConfig;
use super::CommandError;

pub mod pages;
pub mod styles;
pub mod components;

enum FileType {
    Page,
    Style,
    Component,
}

struct NewCommandConfig {
    /// Is it a page, a component or a stylesheet?
    file_type: FileType,
    /// The path the user passed as the second parameter
    file_name: String,
    /// File's parent(s) folder(s)
    target_folder: PathBuf
}

impl NewCommandConfig {
    fn build(base_config: &BaseConfig) -> Result<NewCommandConfig, CommandError> {
        let file_type: FileType;

        match base_config.params[0].as_str() {
            "page" => file_type = FileType::Page,
            "style" => file_type = FileType::Style,
            "component" => file_type = FileType::Component,
            _ => {
                return Err(CommandError {
                    message: String::from(
                        "Wrong file type. Only page, style and component files are allowed",
                    ),
                });
            }
        };

        let file_name = base_config.params[1].trim_matches(|c| c == '/' || c == '\\' ).to_owned();
        let has_src_folder = fs::read_dir("src").is_ok();
        let mut target_folder = PathBuf::from("");

        if has_src_folder {
            target_folder.push("src");
        }

        Ok(NewCommandConfig {
            file_type,
            file_name,
            target_folder
        })
    }
}

pub fn create_file(base_config: BaseConfig) -> Result<(), Box<dyn Error>> {
    // Get the new file config from the base config
    let mut command_config = NewCommandConfig::build(&base_config)?;

    #[allow(unused)]
    match command_config.file_type {
        FileType::Page => {
            pages::create(&mut command_config.target_folder, &mut command_config.file_name)?;
        },
        FileType::Style => {
            styles::create(&mut command_config.target_folder, &mut command_config.file_name)?;
        },
        FileType::Component => {
            components::create(&mut command_config.target_folder, &mut command_config.file_name)?;
        }
    };

    Ok(())
}
