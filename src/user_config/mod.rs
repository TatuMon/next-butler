use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    constants::{CONFIG_FILE_NAME, NEXT_BUTLER_DIR},
    helpers::file_helper::json_file_to_struct, commands::new_command::new_page::PageExtension,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserNewPageConfig {
    /// Create files as typescript
    pub typescript: Option<bool>,
    /// Create files as .jsx (or .tsx if typescript is true)
    pub jsx: Option<bool>,
    /// Which custom template to use by default
    pub template: Option<String>,
    /// Which custom template to use by default
    pub api_template: Option<String>,
    /// Create page based on the old page router
    pub page_router: Option<bool>,
}

impl UserNewPageConfig {
    pub fn guess_extension(&self) -> PageExtension {
        let use_ts = self.typescript.unwrap_or(false);
        let use_jsx = self.jsx.unwrap_or(false);

        if use_ts && use_jsx {
            PageExtension::Tsx
        } else if use_ts && !use_jsx {
            PageExtension::Ts
        } else if !use_ts && use_jsx {
            PageExtension::Jsx
        } else {
            PageExtension::Js
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserNewComponentConfig {
    /// Create files as typescript
    typescript: Option<bool>,
    /// Create files as .jsx (or .tsx if typescript is true)
    jsx: Option<bool>,
    /// Where to save the new components
    folder: Option<String>,
    /// Which custom template to use by default
    template: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserNewStyleConfig {
    /// Which extension to use
    extension: Option<String>,
    /// Which custom template to use by default
    template: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct New {
    page: Option<UserNewPageConfig>,
    style: Option<UserNewStyleConfig>,
    component: Option<UserNewComponentConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
/// The configuration provided by the user.
///
/// Can be either defined in the configuration file or via
/// the tool options.
pub struct UserConfig {
    new: Option<New>,
}

impl UserConfig {
    pub fn get() -> Result<Self, String> {
        let config_file = PathBuf::from(format!("{}{}", NEXT_BUTLER_DIR, CONFIG_FILE_NAME));

        Ok(json_file_to_struct(&config_file)
            .map_err(|err| format!("Custom configuration error: {}", err.to_string()))?)
    }

    pub fn get_new_cmd_config(self) -> Option<New> {
        self.new
    }
}

impl New {
    pub fn get_page_config(self) -> Option<UserNewPageConfig> {
        self.page
    }
    pub fn get_component_config(self) -> Option<UserNewComponentConfig> {
        self.component
    }
    pub fn get_style_config(self) -> Option<UserNewStyleConfig> {
        self.style
    }
}
