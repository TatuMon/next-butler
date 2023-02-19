use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::Path,
    str,
};

use json::JsonValue;

pub mod new_page_config;
pub mod new_style_config;
pub mod new_component_config;

fn get_json_config() -> Result<JsonValue, Box<dyn Error>> {
    let json_path = Path::new("./nextbutler.json");
    let json_config_vec_u8 = fs::read(json_path)?;
    let json_config_str = str::from_utf8(&json_config_vec_u8)?;
    let parsed_json = json::parse(json_config_str)?;

    if !parsed_json.is_object() {
        println!("Found configuration file but with a wrong structure");
        return Err(Box::new(JsonConfigError::new(String::from(
            "Wrong configuration structure",
        ))));
    }

    Ok(parsed_json)
}

fn get_new_command_config() -> Result<JsonValue, Box<dyn Error>> {
    let mut json_config = get_json_config()?;

    if !json_config.has_key("new")
        || json_config["new"].is_null()
        || !json_config["new"].is_object()
    {
        return Err(Box::new(JsonConfigError::new(String::from(
            "Configuration for 'new' command was not found",
        ))));
    }

    Ok(json_config["new"].take())
}

pub fn get_new_pages_config() -> Result<JsonValue, Box<dyn Error>> {
    let mut new_command_config = get_new_command_config()?;

    if !new_command_config.has_key("page")
        || new_command_config["page"].is_null()
        || !new_command_config["page"].is_object()
    {
        return Err(Box::new(JsonConfigError::new(String::from(
            "Configuration for 'new page' command was not found",
        ))));
    }

    Ok(new_command_config["page"].take())
}

pub fn get_new_styles_config() -> Result<JsonValue, Box<dyn Error>> {
    let mut new_command_config = get_new_command_config()?;

    if !new_command_config.has_key("style")
        || new_command_config["style"].is_null()
        || !new_command_config["style"].is_object()
    {
        return Err(Box::new(JsonConfigError::new(String::from(
            "Configuration for 'new style' command was not found",
        ))));
    }

    Ok(new_command_config["style"].take())
}

pub fn get_new_components_config() -> Result<JsonValue, Box<dyn Error>> {
    let mut new_command_config = get_new_command_config()?;

    if !new_command_config.has_key("component")
        || new_command_config["component"].is_null()
        || !new_command_config["component"].is_object()
    {
        return Err(Box::new(JsonConfigError::new(String::from(
            "Configuration for 'new component' command was not found",
        ))));
    }

    Ok(new_command_config["component"].take())
}

#[derive(Debug)]
pub struct JsonConfigError {
    message: String,
}

impl std::error::Error for JsonConfigError {}

impl JsonConfigError {
    pub fn new(message: String) -> JsonConfigError {
        JsonConfigError { message: message }
    }
}

impl Display for JsonConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}
