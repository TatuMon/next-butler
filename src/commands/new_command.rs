use std::fs::{self, create_dir_all};
use std::path::PathBuf;
use std::{error::Error};

use super::super::Config;
use super::CommandError;

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
    fn build(config: &Config) -> Result<NewCommandConfig, Box<dyn Error>> {
        let file_type: FileType;

        match config.params[0].as_str() {
            "page" => file_type = FileType::Page,
            "style" => file_type = FileType::Style,
            "component" => file_type = FileType::Component,
            _ => {
                return Err(Box::new(CommandError {
                    message: String::from(
                        "Wrong file type. Only page, style and component files are allowed",
                    ),
                }));
            }
        };

        let file_name = config.params[1].clone();
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

pub fn create_file(config: Config) -> Result<(), Box<dyn Error>> {
    let mut command_config = NewCommandConfig::build(&config)?;

    #[allow(unused)]
    match command_config.file_type {
        FileType::Page => {
            command_config.target_folder.push("pages");
            fs::create_dir(&command_config.target_folder);
            command_config.file_name.push_str(".tsx");
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

    let final_path = command_config.target_folder.join(&command_config.file_name);

    if let Some(parents) = final_path.parent() {
        create_dir_all(parents)?;
    }

    fs::write(final_path, b"This is a test")?;

    Ok(())
}
