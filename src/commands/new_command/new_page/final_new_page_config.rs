use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR};

use clap::ArgMatches;
use path_clean::PathClean;

use crate::{
    helpers::file_helper,
    react_extension::ReactExtension,
    template::{template_variables::TemplateVariables, Template},
    user_config::{UserConfig, UserNewPageConfig},
    CreateableFileType,
};

pub struct FinalNewPageConfig {
    /// Where the new page will be located
    pub page_final_path: PathBuf,
    /// Template to be used
    pub template: Template,
}

impl FinalNewPageConfig {
    pub fn new(page_args: &ArgMatches) -> Result<Self, String> {
        let usr_page_cfg = UserConfig::get()?.get_page_config();
        let path_arg = PathBuf::from(page_args.get_one::<String>("page_path").unwrap()).clean();
        let page_type = if Self::is_api(&path_arg) {
            CreateableFileType::ApiPage
        } else {
            CreateableFileType::Page
        };
        let filestem = path_arg.file_stem().ok_or("Must specify the page's name")?;
        let template = Self::get_template(
            page_args.get_one::<String>("template"),
            &usr_page_cfg,
            &page_type,
            &TemplateVariables {
                name: filestem.to_string_lossy().to_string().as_str(),
            },
        )?;
        let page_extension =
            Self::get_extension_to_use(page_args, &usr_page_cfg, &page_type, &path_arg);
        let use_page_router = Self::use_page_router(
            page_args.get_flag("page-router"),
            page_args.get_flag("app-router"),
            &usr_page_cfg,
        );
        let page_final_path =
            Self::get_page_final_path(path_arg.to_owned(), &page_extension, use_page_router)?;

        Ok(Self {
            page_final_path,
            template,
        })
    }

    /// Returns the final path of the page (Inside src/pages/ or /pages,
    /// depending on the project), with the correct file extension
    fn get_page_final_path(
        page_path: PathBuf,
        extension: &ReactExtension,
        use_page_router: bool,
    ) -> Result<PathBuf, String> {
        let extension_str: &str = extension.into();
        let mut final_page_path = Self::setup_page_path(page_path, use_page_router)?;

        if final_page_path.set_extension(extension_str) {
            Ok(final_page_path)
        } else {
            Err(String::from("Error setting the extension"))
        }
    }

    /// Set the parents to page_path, based on the correct router (app or page router)
    fn setup_page_path(path_arg: PathBuf, use_page_router: bool) -> Result<PathBuf, String> {
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
                Err(String::from("Couldn't find destination folder"))
            } else {
                Ok(final_path.join(path_arg))
            }
        } else {
            final_path.push("app/");
            if !final_path.exists() {
                return Err(String::from("Couldn't find destination folder"));
            }

            Ok(final_path.join(format!(
                "{}/page",
                path_arg
                    .file_stem()
                    .ok_or(String::from("Must specify the page's name"))?
                    .to_string_lossy()
            )))
        }
    }

    /// Returns true if the name starts with
    /// "api/"
    fn is_api(page_name: &Path) -> bool {
        match page_name.strip_prefix(MAIN_SEPARATOR_STR) {
            Ok(trimmed) => trimmed.starts_with("api/") || trimmed.starts_with("api\\"),
            Err(_) => page_name.starts_with("api/") || page_name.starts_with("api\\"),
        }
    }

    fn get_template(
        template_arg: Option<&String>,
        user_new_page_config: &UserNewPageConfig,
        page_type: &CreateableFileType,
        template_vars: &TemplateVariables,
    ) -> Result<Template, String> {
        if let Some(template_name) = template_arg {
            Template::get_custom_template(template_name, page_type, template_vars)
        } else {
            match page_type {
                CreateableFileType::Page => {
                    if let Some(template_name) = &user_new_page_config.template {
                        Template::get_custom_template(template_name, page_type, template_vars)
                    } else {
                        Ok(Template::get_default_template(page_type))
                    }
                }
                CreateableFileType::ApiPage => {
                    if let Some(template_name) = &user_new_page_config.api_template {
                        Template::get_custom_template(template_name, page_type, template_vars)
                    } else {
                        Ok(Template::get_default_template(page_type))
                    }
                }
                _ => Err(String::from("Incorrect file type.")),
            }
        }
    }

    fn get_extension_to_use(
        page_args: &ArgMatches,
        user_new_page_config: &UserNewPageConfig,
        page_type: &CreateableFileType,
        path_arg: &Path,
    ) -> ReactExtension {
        if let Some(path_arg_extension) = path_arg.extension() {
            return path_arg_extension.into();
        }

        let js_flag = page_args.get_flag("js");
        let ts_flag = page_args.get_flag("ts");
        let jsx_flag = page_args.get_flag("jsx");
        let tsx_flag = page_args.get_flag("tsx");

        if !js_flag && !ts_flag && !jsx_flag && !tsx_flag {
            let usr_cfg_ts = user_new_page_config.typescript.unwrap_or(false);
            let usr_cfg_jsx = user_new_page_config.jsx.unwrap_or(false);
            let is_api = matches!(page_type, CreateableFileType::ApiPage);

            if usr_cfg_ts && usr_cfg_jsx && !is_api {
                "tsx".into()
            } else if usr_cfg_ts && (!usr_cfg_jsx || is_api) {
                "ts".into()
            } else if !usr_cfg_ts && usr_cfg_jsx && !is_api {
                "jsx".into()
            } else {
                "js".into()
            }
        } else {
            ReactExtension::guess(
                js_flag,
                ts_flag,
                jsx_flag,
                tsx_flag,
                None::<UserNewPageConfig>,
            )
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
}
