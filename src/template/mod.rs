use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    constants::NEXT_BUTLER_DIR,
    helpers::file_helper::{self, get_file_stem_occurrences},
    CreateableFileType,
};

use self::{
    default_templates::{
        DEFAULT_API_PAGE_TEMPLATE, DEFAULT_COMPONENT_TEMPLATE, DEFAULT_PAGE_TEMPLATE,
        DEFAULT_STYLESHEET_TEMPLATE,
    },
    template_variables::TemplateVariables,
};

pub mod default_templates;
pub mod template_variables;

/// This pattern will be replaced by the name given to the file
const NAME_PATTERN: &str = "NNNN";

/// If a custom template is used, path will be None
pub struct Template {
    pub path: Option<PathBuf>,
    pub content: Vec<u8>,
}

impl Template {
    pub fn get_custom_template(
        template_name: &str,
        file_type: &CreateableFileType,
        template_vars: &TemplateVariables,
    ) -> Result<Template, String> {
        let template_arg_path = PathBuf::from(template_name);
        let custom_templates_dir = Self::get_custom_templates_path(file_type);

        // If the user specified the custom template extension, directly search the
        // file
        if template_arg_path.extension().is_some() {
            let template_complete_path = custom_templates_dir.join(template_arg_path);
            // Check if the file exists
            if template_complete_path.is_file() {
                Ok(Template {
                    content: Self::get_formatted_template_content(
                        &template_complete_path,
                        template_vars,
                    )?,
                    path: Some(template_complete_path),
                })
            } else {
                Err(String::from("Couldn't find the provided custom template"))
            }
        } else {
            // Else, search all the custom templates with the given name
            match template_arg_path.file_stem() {
                Some(template_stem) => {
                    let found_templates =
                        get_file_stem_occurrences(template_stem, &custom_templates_dir)?;

                    // If none is found, return an error
                    if found_templates.is_empty() {
                        Err(String::from("Couldn't find the provided custom template"))
                    } else if found_templates.len() > 1 {
                        // If there is more than one, return an error
                        Err(String::from("Found multiple custom templates with the given name. Please specify the extension"))
                    } else {
                        // If there is only one, use that template
                        if let Some(path) = found_templates.first() {
                            Ok(Template {
                                path: Some(path.to_owned()),
                                content: fs::read(path).map_err(|err| {
                                    format!(
                                        "Error reading custom template: {}.{}",
                                        template_arg_path.to_string_lossy(),
                                        err
                                    )
                                })?,
                            })
                        } else {
                            Err(String::from("Couldn't find the provided custom template"))
                        }
                    }
                }
                None => Err(String::from("Wrong custom template name")),
            }
        }
    }

    pub fn get_default_template(file: &CreateableFileType) -> Template {
        let template_content = match file {
            CreateableFileType::Page => DEFAULT_PAGE_TEMPLATE,
            CreateableFileType::ApiPage => DEFAULT_API_PAGE_TEMPLATE,
            CreateableFileType::Stylesheet => DEFAULT_STYLESHEET_TEMPLATE,
            CreateableFileType::Component => DEFAULT_COMPONENT_TEMPLATE,
        };

        Template {
            path: None,
            content: template_content.as_bytes().to_owned(),
        }
    }

    fn get_custom_templates_path(file: &CreateableFileType) -> PathBuf {
        let mut custom_templates_path =
            PathBuf::from(format!("{}/{}/", NEXT_BUTLER_DIR, "templates"));
        match file {
            CreateableFileType::Page => custom_templates_path.push("pages/"),
            CreateableFileType::ApiPage => custom_templates_path.push("api-pages/"),
            CreateableFileType::Stylesheet => custom_templates_path.push("stylesheets/"),
            CreateableFileType::Component => custom_templates_path.push("components/"),
        }

        custom_templates_path
    }

    pub fn create_pages_templates<P>(pages_templates_dir: P) -> Result<(), String>
    where
        P: AsRef<Path>,
    {
        fs::create_dir_all(pages_templates_dir.as_ref())
            .map_err(|err| format!("Error creating page templates folder: {}", err))
            .and_then(|()| {
                file_helper::create(
                    &PathBuf::from(pages_templates_dir.as_ref()).join("default.jsx"),
                    DEFAULT_PAGE_TEMPLATE.as_bytes().to_vec(),
                )
            })
            .and_then(|()| {
                file_helper::create(
                    &PathBuf::from(pages_templates_dir.as_ref()).join("api/default.js"),
                    DEFAULT_API_PAGE_TEMPLATE.as_bytes().to_vec(),
                )
            })?;

        Ok(())
    }

    pub fn create_components_templates<P>(components_tempaltes_dir: P) -> Result<(), String>
    where
        P: AsRef<Path>,
    {
        fs::create_dir_all(components_tempaltes_dir.as_ref())
            .map_err(|err| format!("Error creating component templates folder: {}", err))
            .and_then(|()| {
                file_helper::create(
                    &PathBuf::from(components_tempaltes_dir.as_ref()).join("default.jsx"),
                    DEFAULT_COMPONENT_TEMPLATE.as_bytes().to_vec(),
                )
            })?;

        Ok(())
    }

    pub fn create_stylesheets_templates<P>(stylesheets_templates_dir: P) -> Result<(), String>
    where
        P: AsRef<Path>,
    {
        fs::create_dir_all(stylesheets_templates_dir.as_ref())
            .map_err(|err| format!("Error creating stylesheet templates folder: {}", err))
            .and_then(|()| {
                file_helper::create(
                    &PathBuf::from(stylesheets_templates_dir.as_ref()).join("default.css"),
                    DEFAULT_STYLESHEET_TEMPLATE.as_bytes().to_vec(),
                )
            })?;

        Ok(())
    }

    pub fn get_formatted_template_content(
        template_path: &PathBuf,
        template_vars: &TemplateVariables,
    ) -> Result<Vec<u8>, String> {
        let original_content = fs::read_to_string(template_path).map_err(|_| {
            format!(
                "Error reading template content: {}",
                template_path.to_string_lossy()
            )
        })?;

        Ok(Self::format_template_content(
            &original_content,
            template_vars,
        ))
    }

    fn format_template_content(
        original_content: &str,
        template_variables: &TemplateVariables,
    ) -> Vec<u8> {
        original_content
            .replace(NAME_PATTERN, template_variables.name)
            .into()
    }
}
