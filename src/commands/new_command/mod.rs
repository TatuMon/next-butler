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
    file_type: FileType,
    file_name: String,
    target_folder: PathBuf,
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

        let file_name = base_config.params[1].clone();
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
    let mut command_config = NewCommandConfig::build(&base_config)?;

    #[allow(unused)]
    match command_config.file_type {
        FileType::Page => {
            pages::create(&mut command_config.target_folder, &mut command_config.file_name)?;
        },
        FileType::Style => {
            command_config.target_folder.push("styles");
            fs::create_dir(&command_config.target_folder);
            command_config.file_name.push_str(".scss");
        },
        FileType::Component => {
            command_config.target_folder.push("components");
            fs::create_dir(&command_config.target_folder);
            command_config.file_name.push_str(".tsx");
        }
    };

    Ok(())
}
