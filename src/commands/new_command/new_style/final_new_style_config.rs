use std::{ffi::OsStr, path::PathBuf};

use clap::ArgMatches;

use crate::{
    helpers::file_helper,
    template::{template_variables::TemplateVariables, Template},
    user_config::{UserConfig, UserNewStyleConfig},
    CreateableFileType,
};

pub struct FinalNewStyleConfig {
    /// Where the new component will be located
    pub style_final_path: PathBuf,
    /// Template to be used
    pub template: Template,
}

impl FinalNewStyleConfig {
    pub fn new(style_args: &ArgMatches) -> Result<Self, String> {
        let usr_style_cfg = if let Some(usr_new_cmd_cfg) = UserConfig::get()?.get_new_cmd_config() {
            usr_new_cmd_cfg.get_style_config()
        } else {
            None
        };
        let path_arg = PathBuf::from(style_args.get_one::<String>("component_path").unwrap());
        let filestem = path_arg
            .file_stem()
            .ok_or(format!("Must specify the stylesheet's name"))?;
        let file_type = CreateableFileType::Stylesheet;
        let template = Self::get_template(
            style_args.get_one::<String>("template"),
            &usr_style_cfg,
            &file_type,
            &TemplateVariables {
                name: filestem.to_string_lossy().to_string().as_str(),
            },
        )?;
        // Unwrap because I defined a default value
        let style_extension = style_args.get_one::<String>("extension").unwrap();
        let destination_folder = style_args.get_one::<String>("folder");

        let style_final_path = Self::get_style_final_path(
            path_arg.to_owned(),
            style_extension,
            filestem,
            destination_folder,
        )?;

        Ok(Self {
            style_final_path,
            template,
        })
    }

    fn get_style_final_path(
        path_arg: PathBuf,
        extension: &str,
        filestem: &OsStr,
        destination_folder: Option<&String>,
    ) -> Result<PathBuf, String> {
        let path_arg = path_arg
            .strip_prefix("/")
            .unwrap_or(path_arg.as_path())
            .to_path_buf();

        // Base path of the new stylesheet
        let mut final_path = PathBuf::new();

        if file_helper::is_src_present()? {
            final_path.push("src/");
        }

        if let Some(destination_folder) = destination_folder {
            final_path.push(destination_folder);
            if !final_path.exists() {
                return Err(String::from("Couldn't find destination folder"));
            }
        }

        final_path.push(path_arg);
        final_path.push(format!("{}", filestem.to_string_lossy()));
        final_path.set_extension(extension);

        Ok(final_path)
    }

    fn get_template(
        template_arg: Option<&String>,
        user_new_style_config: &Option<UserNewStyleConfig>,
        file_type: &CreateableFileType,
        template_vars: &TemplateVariables,
    ) -> Result<Template, String> {
        if let Some(template_name) = template_arg {
            Template::get_custom_template(template_name, file_type, template_vars)
        } else if let Some(user_new_style_config) = user_new_style_config {
            if let Some(template_name) = &user_new_style_config.template {
                Template::get_custom_template(template_name, file_type, template_vars)
            } else {
                Ok(Template::get_default_template(file_type))
            }
        } else {
            Ok(Template::get_default_template(&file_type))
        }
    }
}
