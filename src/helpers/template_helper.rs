use super::file_helper::get_file_stem_occurrences;
use crate::{
    constants::{
        DEFAULT_API_PAGE_TEMPLATE, DEFAULT_COMPONENT_TEMPLATE, DEFAULT_PAGE_TEMPLATE,
        DEFAULT_STYLESHEET_TEMPLATE, NEXT_BUTLER_DIR,
    },
    CreateableFileType,
};
use std::{fs, path::PathBuf};

/// This pattern will be replaced by the name given to the file
const NAME_PATTERN: &str = "NNNN";

/// If a custom template is used, path will be None
pub struct Template {
    pub path: Option<PathBuf>,
    pub content: Vec<u8>,
}

pub fn get_custom_template(
    template_name: &str,
    file_type: &CreateableFileType,
) -> Result<Template, String> {
    let template_arg_path = PathBuf::from(template_name);
    let custom_templates_dir = get_custom_templates_path(&file_type);

    // If the user specified the custom template extension, directly search the
    // file
    if template_arg_path.extension().is_some() {
        let template_complete_path = custom_templates_dir.join(template_arg_path);
        // Check if the file exists
        if template_complete_path.is_file() {
            Ok(Template {
                path: Some(template_complete_path),
                content: fs::read(template_complete_path).map_err(|err| {
                    format!(
                        "Error reading custom template: {}.{}",
                        template_arg_path.to_string_lossy(),
                        err.to_string()
                    )
                })?,
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
                                    err.to_string()
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
    let mut custom_templates_path = PathBuf::from(format!("{}/{}/", NEXT_BUTLER_DIR, "templates"));
    match file {
        CreateableFileType::Page => custom_templates_path.push("pages/"),
        CreateableFileType::ApiPage => custom_templates_path.push("api-pages/"),
        CreateableFileType::Stylesheet => custom_templates_path.push("stylesheets/"),
        CreateableFileType::Component => custom_templates_path.push("components/"),
    }

    custom_templates_path
}

pub fn create_default_page_template<P>(page_templates_dir: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    //Creates page templates dir
    fs::create_dir_all(page_templates_dir.as_ref().clone())
        .map_err(|err| format!("Error creating page templates folder: {}", err.to_string()))?;

    Ok(())
}

pub fn create_default_component_template<P>(page_templates_dir: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    //Creates page templates dir
    fs::create_dir_all(page_templates_dir.as_ref().clone())
        .map_err(|err| format!("Error creating component templates folder: {}", err.to_string()))?;

    Ok(())
}
pub fn create_default_stylesheet_template<P>(page_templates_dir: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    //Creates page templates dir
    fs::create_dir_all(page_templates_dir.as_ref().clone())
        .map_err(|err| format!("Error creating stylesheet templates folder: {}", err.to_string()))?;

    Ok(())
}


