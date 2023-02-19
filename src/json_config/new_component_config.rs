use crate::helpers::str_helper::{to_folder_name};

use super::*;

pub struct NewComponentConfig {
    pub typescript: bool,
    pub use_jsx: bool,
    pub folder: String,
}

impl NewComponentConfig {
    pub fn build() -> Result<NewComponentConfig, Box<dyn Error>> {
        let mut new_command_config = get_new_command_config()?;

        if !new_command_config.has_key("component")
            || new_command_config["component"].is_null()
            || !new_command_config["component"].is_object()
        {
            return Err(Box::new(JsonConfigError::new(String::from(
                "Configuration for 'new component' command was not found",
            ))));
        }

        let component_config = new_command_config["component"].take();

        let typescript = match component_config["typescript"].as_bool() {
            Some(val) => val,
            None => false,
        };

        let use_jsx = match component_config["use_jsx"].as_bool() {
            Some(val) => val,
            None => false,
        };

        let folder = match component_config["folder"].as_str() {
            Some(val) => to_folder_name(val),
            None => String::from("components"),
        };

        Ok(NewComponentConfig {
            typescript,
            use_jsx,
            folder,
        })
    }
}
