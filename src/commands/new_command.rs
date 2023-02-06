use std::fs;
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
    file_name: PathBuf,
    base_target_folder: PathBuf,
}

impl NewCommandConfig {
    fn build(config: &Config) -> Result<NewCommandConfig, Box<dyn Error>> {
        let file_type: FileType;

        match config.params[0].as_str() {
            "page" => file_type = FileType::Page,
            "style" => file_type = FileType::Style,
            "Component" => file_type = FileType::Component,
            _ => {
                return Err(Box::new(CommandError {
                    message: String::from(
                        "Wrong file type. Only page, style and component files are allowed",
                    ),
                }));
            }
        };

        let file_name = PathBuf::from(&config.params[1]);
        let has_src_folder = fs::read_dir("srcc").is_ok();
        let mut base_target_folder = PathBuf::from("");

        if has_src_folder {
            base_target_folder.push("/src");
        }

        Ok(NewCommandConfig {
            file_type,
            file_name,
            base_target_folder
        })
    }
}

pub fn create_file(config: Config) -> Result<(), Box<dyn Error>> {
    let mut command_config = NewCommandConfig::build(&config)?;

    #[allow(unused)]
    match command_config.file_type {
        FileType::Page => {
            fs::create_dir("pages");
            command_config.base_target_folder.push("pages");
            command_config.file_name.set_extension("tsx");
        },
        FileType::Style => {
            fs::create_dir("styles");
            command_config.base_target_folder.push("styles");
            command_config.file_name.set_extension("scss");
        },
        FileType::Component => {
            fs::create_dir("components");
            command_config.base_target_folder.push("components");
            command_config.file_name.set_extension("tsx");
        }
    };

    let final_path = command_config.base_target_folder.join(command_config.file_name);

    println!("{}", final_path.display());

    fs::write(final_path, b"Hola puos")?;

    Ok(())
}
