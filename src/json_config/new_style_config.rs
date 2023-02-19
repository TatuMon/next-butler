use crate::helpers::str_helper::to_ext;

use super::*;

pub struct NewStyleConfig {
    pub ext: String,
}

impl NewStyleConfig {
    pub fn build() -> Result<NewStyleConfig, Box<dyn Error>> {
        let mut new_command_config = get_new_command_config()?;

        if !new_command_config.has_key("style")
            || new_command_config["style"].is_null()
            || !new_command_config["style"].is_object()
        {
            return Err(Box::new(JsonConfigError::new(String::from(
                "Configuration for 'new page' command was not found",
            ))));
        }

        let page_config = new_command_config["style"].take();

        let ext = match page_config["ext"].as_str() {
            Some(val) => {
                to_ext(val)
            },
            None => String::from(".css")
        };

        Ok(NewStyleConfig {
            ext
        })
    }
}