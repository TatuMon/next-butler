pub mod components;
pub mod new_command_config;
pub mod pages;
pub mod styles;

use super::{super::BaseConfig, command_error::CommandError};
use new_command_config::{FileType, NewCommandConfig};
use std::error::Error;

pub fn create_file(base_config: BaseConfig) -> Result<(), Box<dyn Error>> {
    // If there are options, we process it first
    if !base_config.options.is_empty() {
        process_options(&base_config.options);
    }

    // Builds the new file config from the base config
    let mut command_config = NewCommandConfig::build(&base_config)?;

    #[allow(unused)]
    match command_config.file_type {
        FileType::Page => {
            pages::create(&mut command_config, &base_config.options)?;
        }
        FileType::Style => {
            styles::create(&mut command_config, &base_config.options)?;
        }
        FileType::Component => {
            components::create(&mut command_config, &base_config.options)?;
        }
        _ => {
            return Err(Box::new(CommandError::invalid_file_type()));
        }
    };

    Ok(())
}

fn process_options(options: &Vec<String>) {
    for option in options {
        match option.as_str() {
            "--help" => show_new_command_help(),
            _ => (),
        }
    }
}

fn show_new_command_help() {
    println!("This shows the new command help text");
}
