use std::path::{Path, PathBuf};

use clap::ArgMatches;
use path_clean::PathClean;

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
        let usr_style_cfg = UserConfig::get()?.get_style_config();

        // ARGUMENTS
        let mut path_arg = PathBuf::from(style_args.get_one::<String>("style_name").unwrap());
        file_helper::rm_double_dots_from_path_buf(&mut path_arg);
        path_arg = path_arg.clean();

        let style_extension = if let Some(path_arg_extension) = path_arg.extension() {
            path_arg_extension.to_string_lossy().to_string()
        } else {
            match style_args.get_one::<String>("extension") {
                Some(extension) => extension.to_owned(),
                None => usr_style_cfg
                    .extension
                    .to_owned()
                    .unwrap_or(String::from("css")),
            }
        };
        let folder = match style_args.get_one::<String>("folder") {
            Some(folder) => folder.to_owned(),
            None => usr_style_cfg
                .folder
                .to_owned()
                .unwrap_or(String::from("css")),
        };

        let file_type = CreateableFileType::Stylesheet;
        let filestem = path_arg
            .file_stem()
            .ok_or("Must specify stylesheet name")?
            .to_string_lossy();
        let template = Self::get_template(
            style_args.get_one::<String>("template"),
            &usr_style_cfg,
            &file_type,
            &TemplateVariables {
                name: filestem.to_string().as_str(),
            },
        )?;

        let style_final_path =
            Self::get_style_final_path(&path_arg, &style_extension, &filestem, &folder)?;

        Ok(Self {
            style_final_path,
            template,
        })
    }

    fn get_style_final_path(
        path_arg: &Path,
        extension: &str,
        filestem: &str,
        destination_folder: &str,
    ) -> Result<PathBuf, String> {
        let path_arg = path_arg.strip_prefix("/").unwrap_or(path_arg).to_path_buf();

        // Base path of the new stylesheet
        let mut final_path = PathBuf::new();

        if file_helper::is_src_present()? {
            final_path.push("src/");
        }

        final_path.push(destination_folder);
        if !final_path.exists() {
            return Err(String::from("Couldn't find destination folder"));
        }

        final_path.push(path_arg);
        final_path.push(filestem);
        final_path.set_extension(extension);

        Ok(final_path)
    }

    fn get_template(
        template_arg: Option<&String>,
        user_new_style_config: &UserNewStyleConfig,
        file_type: &CreateableFileType,
        template_vars: &TemplateVariables,
    ) -> Result<Template, String> {
        if let Some(template_name) = template_arg {
            Template::get_custom_template(template_name, file_type, template_vars)
        } else if let Some(template_name) = &user_new_style_config.template {
            Template::get_custom_template(template_name, file_type, template_vars)
        } else {
            Ok(Template::get_default_template(file_type))
        }
    }
}
