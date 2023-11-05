use std::path::PathBuf;

use clap::ArgMatches;

use crate::{
    helpers::{
        file_helper::get_name_or_err,
        template_helper::{get_page_content, get_template, Template},
    },
    user_config::{UserConfig, UserNewPageConfig},
    CreateableFileType,
};

use super::{get_page_final_path, is_api, PageExtension};

pub struct FinalNewPageConfig {
    /// Where the new page will be located
    pub page_final_path: PathBuf,
    /// Final content of the page
    pub page_content: Vec<u8>,
}

impl FinalNewPageConfig {
    pub fn new(page_args: &ArgMatches) -> Result<Self, String> {
        let usr_page_cfg = if let Some(usr_new_cmd_cfg) = UserConfig::get()?.get_new_cmd_config() {
            usr_new_cmd_cfg.get_page_config()
        } else {
            None
        };

        let path_arg = PathBuf::from(page_args.get_one::<String>("page_path").unwrap());

        let page_type = if is_api(&path_arg) {
            CreateableFileType::ApiPage
        } else {
            CreateableFileType::Page
        };

        let template = Self::get_template_to_create(
            page_args.get_one::<String>("template"),
            &usr_page_cfg,
            &page_type,
        )?;

        let page_extension = Self::get_extension_to_use(
            page_args,
            &usr_page_cfg,
            &template,
            &page_type
        );

        let page_final_path = get_page_final_path(path_arg.to_owned(), page_extension)?;
        let page_name = get_name_or_err(&page_final_path)?;
        let page_content = get_page_content(page_name, template)?;

        Ok(Self {
            page_final_path,
            page_content,
        })
    }

    fn get_template_to_create(
        template_arg: Option<&String>,
        user_new_page_config: &Option<UserNewPageConfig>,
        page_type: &CreateableFileType,
    ) -> Result<Template, String> {
        if template_arg.is_some() {
            get_template(&template_arg, &page_type)
        } else if let Some(user_new_page_config) = user_new_page_config {
            match page_type {
                CreateableFileType::Page => get_template(&user_new_page_config.template, &page_type),
                CreateableFileType::ApiPage => {
                    get_template(&user_new_page_config.api_template, page_type)
                }
                _ => Err(String::from("Incorrect file type.")),
            }
        } else {
            get_template::<String>(&None, &page_type)
        }
    }

    fn get_extension_to_use(
        page_args: &ArgMatches,
        user_new_page_config: &Option<UserNewPageConfig>,
        template: &Template,
        page_type: &CreateableFileType,
    ) -> PageExtension {
        let ts_flag = page_args.get_flag("ts");
        let tsx_flag = page_args.get_flag("jsx");
        let js_flag = page_args.get_flag("js");

        if !(tsx_flag && ts_flag && js_flag) {
            if let Some(user_new_page_config) = user_new_page_config {
                let usr_cfg_ts = user_new_page_config.typescript.unwrap_or(false);
                let usr_cfg_jsx = user_new_page_config.jsx.unwrap_or(false);
                let is_api = match page_type {
                    CreateableFileType::ApiPage => true,
                    _ => false,
                };

                if usr_cfg_ts && usr_cfg_jsx && !is_api {
                    "tsx".into()
                } else if usr_cfg_ts && !usr_cfg_jsx {
                    "ts".into()
                } else if usr_cfg_ts && is_api {
                    "ts".into()
                } else if !usr_cfg_ts && usr_cfg_jsx && !is_api {
                    "jsx".into()
                } else {
                    "js".into()
                }
            } else {
                "js".into()
            }
        } else {
            PageExtension::guess(js_flag, tsx_flag, ts_flag, template)
        }
    }
}
