use std::{fs, path::PathBuf};

use crate::{base_config::BaseConfig, commands::command_error::CommandError};

pub enum FileType {
    Page,
    Style,
    Component,
}

pub struct NewCommandConfig {
    /// Is it a page, a component or a stylesheet?
    pub file_type: FileType,
    /// The path the user passed as the second parameter
    pub file_name: String,
    /// File's parent(s) folder(s)
    pub target_folder: PathBuf,
}

impl NewCommandConfig {
    pub fn build(base_config: &BaseConfig) -> Result<NewCommandConfig, CommandError> {
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

        let file_name = base_config.params[1]
            .trim_matches(|c| c == '/' || c == '\\')
            .to_owned();
        let has_src_folder = fs::read_dir("src").is_ok();
        let mut target_folder = PathBuf::from("");

        if has_src_folder {
            target_folder.push("src");
        }

        Ok(NewCommandConfig {
            file_type,
            file_name,
            target_folder,
        })
    }
}

