use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
    constants::{CONFIG_FILE_NAME, NEXT_BUTLER_DIR},
    helpers::file_helper::json_file_to_struct,
    react_extension::{GuessReactExtension, ReactExtension},
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
    pub fn get_default() -> Self {
        Self {
            typescript: Some(false),
            jsx: Some(true),
            template: None,
            api_template: None,
            page_router: Some(true),
        }
    }
}

impl GuessReactExtension for UserNewPageConfig {
    fn guess_extension(&self) -> ReactExtension {
        let use_ts = self.typescript.unwrap_or(false);
        let use_jsx = self.jsx.unwrap_or(false);

        if use_ts && use_jsx {
            ReactExtension::Tsx
        } else if use_ts && !use_jsx {
            ReactExtension::Ts
        } else if !use_ts && use_jsx {
            ReactExtension::Jsx
        } else {
            ReactExtension::Js
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserNewComponentConfig {
    /// Create files as typescript
    pub typescript: Option<bool>,
    /// Create files as .jsx (or .tsx if typescript is true)
    pub jsx: Option<bool>,
    /// Where to save the new components
    pub folder: Option<String>,
    /// Which custom template to use by default
    pub template: Option<String>,
}

impl UserNewComponentConfig {
    pub fn get_default() -> Self {
        Self {
            typescript: Some(false),
            jsx: Some(true),
            folder: Some(String::from("components")),
            template: None,
        }
    }
}

impl GuessReactExtension for UserNewComponentConfig {
    fn guess_extension(&self) -> ReactExtension {
        let use_ts = self.typescript.unwrap_or(false);
        let use_jsx = self.jsx.unwrap_or(false);

        if use_ts && use_jsx {
            ReactExtension::Tsx
        } else if use_ts && !use_jsx {
            ReactExtension::Ts
        } else if !use_ts && use_jsx {
            ReactExtension::Jsx
        } else {
            ReactExtension::Js
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserNewStyleConfig {
    /// Which extension to use
    pub extension: Option<String>,
    /// Which custom template to use by default
    pub template: Option<String>,
    /// Where the stylesheets should be created
    pub folder: Option<String>,
}

impl UserNewStyleConfig {
    pub fn get_default() -> Self {
        Self {
            extension: Some(String::from("css")),
            folder: Some(String::from("styles")),
            template: None,
        }
    }
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
    /// Returns the user-defined configuration or the default if it doesn't exist
    pub fn get() -> Result<Self, String> {
        let config_file = PathBuf::from(format!("{}{}", NEXT_BUTLER_DIR, CONFIG_FILE_NAME));

        if config_file.exists() {
            json_file_to_struct(&config_file)
                .map_err(|err| format!("Custom configuration error: {}", err))
        } else {
            Ok(Self::get_default())
        }
    }

    pub fn get_new_cmd_config(self) -> Option<New> {
        self.new
    }

    pub fn get_page_config(self) -> UserNewPageConfig {
        if let Some(new_cmd_cfg) = self.new {
            new_cmd_cfg
                .get_page_config()
                .map_or_else(UserNewPageConfig::get_default, |v| v)
        } else {
            UserNewPageConfig::get_default()
        }
    }

    pub fn get_component_config(self) -> UserNewComponentConfig {
        if let Some(new_cmd_cfg) = self.new {
            new_cmd_cfg
                .get_component_config()
                .map_or_else(UserNewComponentConfig::get_default, |v| v)
        } else {
            UserNewComponentConfig::get_default()
        }
    }

    pub fn get_style_config(self) -> UserNewStyleConfig {
        if let Some(new_cmd_cfg) = self.new {
            new_cmd_cfg
                .get_style_config()
                .map_or_else(UserNewStyleConfig::get_default, |v| v)
        } else {
            UserNewStyleConfig::get_default()
        }
    }

    pub fn get_default() -> Self {
        Self {
            new: Some(New {
                page: Some(UserNewPageConfig::get_default()),
                style: Some(UserNewStyleConfig::get_default()),
                component: Some(UserNewComponentConfig::get_default()),
            }),
        }
    }

    pub fn get_default_as_vec() -> Result<Vec<u8>, String> {
        serde_json::to_vec_pretty(&Self::get_default())
            .map_err(|err| format!("Error building the default configuration file: {}", err))
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
