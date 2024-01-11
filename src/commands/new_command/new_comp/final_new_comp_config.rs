use std::path::PathBuf;

use clap::ArgMatches;
use path_clean::PathClean;

use crate::{
    helpers::file_helper,
    react_extension::ReactExtension,
    template::{template_variables::TemplateVariables, Template},
    user_config::{UserConfig, UserNewComponentConfig},
    CreateableFileType,
};

pub struct FinalNewCompConfig {
    /// Where the new component will be located
    pub comp_final_path: PathBuf,
    /// Template to be used
    pub template: Template,
}

impl FinalNewCompConfig {
    pub fn new(comp_args: &ArgMatches) -> Result<Self, String> {
        let usr_comp_cfg = UserConfig::get()?.get_component_config();

        let path_arg =
            PathBuf::from(comp_args.get_one::<String>("component_path").unwrap()).clean();
        let file_type = CreateableFileType::Component;
        let comp_extension =
            Self::get_extension_to_use(comp_args, &usr_comp_cfg, &file_type, &path_arg);
        let destination_folder = match comp_args.get_one::<String>("folder") {
            Some(destination_folder) => destination_folder.to_owned(),
            None => usr_comp_cfg
                .folder
                .clone()
                .unwrap_or(String::from("components")),
        };
        let comp_final_path =
            Self::get_comp_final_path(path_arg.to_owned(), &comp_extension, &destination_folder)?;

        let filestem = path_arg
            .file_stem()
            .ok_or("Must specify the component's name")?;
        let template = Self::get_template(
            comp_args.get_one::<String>("template"),
            &usr_comp_cfg,
            &file_type,
            &TemplateVariables {
                name: filestem.to_string_lossy().to_string().as_str(),
            },
        )?;

        Ok(Self {
            comp_final_path,
            template,
        })
    }

    fn get_comp_final_path(
        path_arg: PathBuf,
        extension: &ReactExtension,
        destination_folder: &String,
    ) -> Result<PathBuf, String> {
        let path_arg = path_arg
            .strip_prefix("/")
            .unwrap_or(path_arg.as_path())
            .to_path_buf();

        if path_arg.ends_with("/") {
            return Err(String::from("Must specify the component's name"));
        }

        // Base path of the new component
        let mut final_path = PathBuf::new();

        if file_helper::is_src_present()? {
            final_path.push("src/");
        }

        final_path.push(destination_folder);
        if !final_path.exists() {
            return Err(String::from("Couldn't find destination folder"));
        }

        final_path.push(path_arg);
        final_path.set_extension(extension);

        Ok(final_path)
    }

    fn get_template(
        template_arg: Option<&String>,
        user_new_comp_config: &UserNewComponentConfig,
        file_type: &CreateableFileType,
        template_vars: &TemplateVariables,
    ) -> Result<Template, String> {
        if let Some(template_name) = template_arg {
            Template::get_custom_template(template_name, file_type, template_vars)
        } else if let Some(template_name) = &user_new_comp_config.template {
            Template::get_custom_template(template_name, file_type, template_vars)
        } else {
            Ok(Template::get_default_template(file_type))
        }
    }

    fn get_extension_to_use(
        page_args: &ArgMatches,
        user_new_comp_config: &UserNewComponentConfig,
        page_type: &CreateableFileType,
        path_arg: &PathBuf,
    ) -> ReactExtension {
        if let Some(path_arg_extension) = path_arg.extension() {
            return path_arg_extension.into();
        }

        let js_flag = page_args.get_flag("js");
        let ts_flag = page_args.get_flag("ts");
        let jsx_flag = page_args.get_flag("jsx");
        let tsx_flag = page_args.get_flag("tsx");

        if !js_flag && !ts_flag && !jsx_flag && !tsx_flag {
            let usr_cfg_ts = user_new_comp_config.typescript.unwrap_or(false);
            let usr_cfg_jsx = user_new_comp_config.jsx.unwrap_or(false);
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
                None::<UserNewComponentConfig>,
            )
        }
    }
}
