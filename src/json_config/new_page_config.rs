use std::error::Error;

pub struct NewPageConfig {
    pub typescript: bool,
    pub use_jsx: bool,
}

impl NewPageConfig {
    /// Builds the configuration for the new page, based on nextbutler.json
    pub fn build() -> Result<NewPageConfig, Box<dyn Error>> {
        let mut new_command_config = super::get_new_command_config()?;

        if !new_command_config.has_key("page")
            || new_command_config["page"].is_null()
            || !new_command_config["page"].is_object()
        {
            return Err(Box::new(super::JsonConfigError::new(String::from(
                "Configuration for 'new page' command was not found",
            ))));
        }

        let page_config = new_command_config["page"].take();

        let typescript = match page_config["typescript"].as_bool() {
            Some(val) => val,
            None => false
        };

        let use_jsx = match page_config["use_jsx"].as_bool() {
            Some(val) => val,
            None => false
        };

        Ok(NewPageConfig {
            typescript,
            use_jsx
        })
    }
}