use std::{
    collections::BTreeMap,
    path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use clap::ArgMatches;
use convert_case::{Case, Casing};
use path_clean::PathClean;

use crate::{
    helpers::file_helper,
    react_extension::ReactExtension,
    template::{get_custom_template, get_default_template, Template},
    user_config::{UserConfig, UserNewPageConfig},
    CreateableFileType,
};

pub struct FinalNewPageConfig<'a> {
    /// Where the new page will be located
    pub page_final_path: PathBuf,
    pub template: Template<'a>,
    pub template_vars: BTreeMap<String, String>,
}

impl<'a> FinalNewPageConfig<'a> {
    pub fn new(page_args: &ArgMatches) -> Result<Self, String> {
        let usr_page_cfg = UserConfig::get()?.get_page_config();
        let mut path_arg = PathBuf::from(page_args.get_one::<String>("page_path").unwrap());
        file_helper::rm_double_dots_from_path_buf(&mut path_arg);
        path_arg = path_arg.clean();

        let page_type = if Self::is_api(&path_arg) {
            CreateableFileType::ApiPage
        } else {
            CreateableFileType::Page
        };
        let use_page_router = Self::use_page_router(
            page_args.get_flag("page-router"),
            page_args.get_flag("app-router"),
            &usr_page_cfg,
        );

        let new_page_name = path_arg
            .file_stem()
            .ok_or("Must specify the page's name")?
            .to_string_lossy()
            .to_case(Case::Pascal);
        let template_vars = BTreeMap::from([("name".to_owned(), new_page_name)]);

        let template = Self::get_template(
            page_args.get_one::<String>("template"),
            &usr_page_cfg,
            &page_type,
        )?;

        let page_final_extension =
            Self::get_extension_to_use(page_args, &usr_page_cfg, &page_type, &template)?;
        let page_final_path =
            Self::setup_page_path(path_arg, use_page_router, &page_final_extension)?;

        Ok(Self {
            page_final_path,
            template,
            template_vars,
        })
    }

    /// Set the parents to page_path, based on the correct router (app or page router)
    fn setup_page_path(
        path_arg: PathBuf,
        use_page_router: bool,
        extension: &ReactExtension
    ) -> Result<PathBuf, String> {
        let path_arg = path_arg
            .strip_prefix("/")
            .unwrap_or(path_arg.as_path())
            .to_path_buf();

        if path_arg.ends_with("/") {
            return Err(String::from("Must specify the page's name"));
        }

        // Base path of the new page
        let mut final_path = PathBuf::new();

        if file_helper::is_src_present()? {
            final_path.push("src/");
        }

        if use_page_router {
            final_path.push("pages/");
            if !final_path.exists() {
                return Err(String::from("Couldn't find destination folder"));
            } else {
                final_path.push(path_arg);
            }
        } else {
            final_path.push("app/");
            if !final_path.exists() {
                return Err(String::from("Couldn't find destination folder"));
            }

            final_path.push(format!("{}/page", path_arg.to_string_lossy()));
        }

        final_path.set_extension(extension);

        Ok(final_path)
    }

    /// Returns true if the name starts with
    /// "api/"
    fn is_api(page_name: &Path) -> bool {
        match page_name.strip_prefix(MAIN_SEPARATOR_STR) {
            Ok(trimmed) => trimmed.starts_with("api/") || trimmed.starts_with("api\\"),
            Err(_) => page_name.starts_with("api/") || page_name.starts_with("api\\"),
        }
    }

    fn get_extension_to_use(
        page_args: &ArgMatches,
        user_new_page_config: &UserNewPageConfig,
        page_type: &CreateableFileType,
        template: &Template
    ) -> Result<ReactExtension, String> {
        let js_flag = page_args.get_flag("js");
        let ts_flag = page_args.get_flag("ts");
        let jsx_flag = page_args.get_flag("jsx");
        let tsx_flag = page_args.get_flag("tsx");

        if !js_flag && !ts_flag && !jsx_flag && !tsx_flag {
            // Get extension from template
            if let Template::Path(tmpl_path) = template {
                let tmpl_stem = tmpl_path.file_stem();
                if let Some(tmpl_stem) = tmpl_stem {
                    if let Some(tmpl_extension) = PathBuf::from(tmpl_stem).extension() {
                        return Ok(tmpl_extension.into());
                    }
                }
            }

            // Get extension from configuration file
            let usr_cfg_ts = user_new_page_config.typescript.unwrap_or(false);
            let usr_cfg_jsx = user_new_page_config.jsx.unwrap_or(false);
            let is_api = matches!(page_type, CreateableFileType::ApiPage);

            if usr_cfg_ts && usr_cfg_jsx && !is_api {
                Ok("tsx".into())
            } else if usr_cfg_ts && (!usr_cfg_jsx || is_api) {
                Ok("ts".into())
            } else if !usr_cfg_ts && usr_cfg_jsx && !is_api {
                Ok("jsx".into())
            } else {
                Ok("js".into())
            }
        } else {
            Ok(ReactExtension::guess(
                js_flag,
                ts_flag,
                jsx_flag,
                tsx_flag,
                None::<UserNewPageConfig>,
            ))
        }
    }

    /// If no argument or configuration is set, use the app router by default
    fn use_page_router(
        page_router_arg: bool,
        app_router_arg: bool,
        user_new_page_config: &UserNewPageConfig,
    ) -> bool {
        if page_router_arg {
            true
        } else if app_router_arg {
            false
        } else if let Some(use_page_router_cfg) = user_new_page_config.page_router {
            use_page_router_cfg
        } else {
            false
        }
    }

    fn get_template(
        template_arg: Option<&String>,
        user_new_comp_config: &UserNewPageConfig,
        file_type: &CreateableFileType,
    ) -> Result<Template<'a>, String> {
        if let Some(template_name) = template_arg {
            get_custom_template(template_name, file_type)
        } else if let Some(template_name) = &user_new_comp_config.template {
            get_custom_template(template_name, file_type)
        } else {
            Ok(get_default_template(file_type))
        }
    }
}
