use convert_case::{Case, Converter};
use std::{fs, path::PathBuf};

use crate::{
    constants::NEXT_BUTLER_DIR, get_out_dir, helpers::file_helper::eq_file_name, CreateableFileType,
};

use super::{file_helper::eq_file_extensions, str_helper::split_last};

/// This pattern will be replaced by the name given to the file
const NAME_PATTERN: &str = "NNNN";

pub fn get_page_content(
    page_name: &str,
    is_api: bool,
    template: Option<&String>,
) -> Result<Vec<u8>, String> {
    let page_template: PathBuf = if is_api {
        get_template(template, CreateableFileType::ApiPage)?
    } else {
        get_template(template, CreateableFileType::Page)?
    };

    match fs::read_to_string(page_template) {
        Ok(template_content) => {
            let converter = Converter::new().to_case(Case::Pascal);
            let converted_page_name = converter.convert(page_name);
            let formatted_template_content =
                template_content.replace(NAME_PATTERN, &converted_page_name[..]);
            Ok(formatted_template_content.as_bytes().to_owned())
        }
        Err(_) => Err(String::from("Couldn't read the page template")),
    }
}

pub fn get_component_content(
    component_name: &str,
    template: Option<&String>,
) -> Result<Vec<u8>, String> {
    let component_template = get_template(template, CreateableFileType::Component)?;

    match fs::read_to_string(component_template) {
        Ok(content) => {
            let conv = Converter::new().to_case(Case::Pascal);
            let final_content = content.replace(NAME_PATTERN, &(conv.convert(component_name))[..]);
            Ok(final_content.as_bytes().to_owned())
        }
        Err(_) => Err(String::from("Couldn't read the component template")),
    }
}

pub fn get_stylesheet_content(
    stylesheet_name: &str,
    template: Option<&String>,
) -> Result<Vec<u8>, String> {
    let stylesheet_template = get_template(template, CreateableFileType::Component)?;

    match fs::read_to_string(stylesheet_template) {
        Ok(content) => {
            let conv = Converter::new().to_case(Case::Pascal);
            let final_content = content.replace(NAME_PATTERN, &(conv.convert(stylesheet_name))[..]);
            Ok(final_content.as_bytes().to_owned())
        }
        Err(_) => Err(String::from("Couldn't read the component template")),
    }
}

fn get_template(template_name: Option<&String>, file: CreateableFileType) -> Result<PathBuf, String> {
    let final_template;
    if let Some(custom_template) = template_name {
        final_template = get_custom_template(custom_template, file);
    } else {
        final_template = Ok(get_default_template(file));
    }

    final_template
}

fn get_custom_template(template_name: &String, file: CreateableFileType) -> Result<PathBuf, String> {
    let template_path = PathBuf::from(template_name);

    let template_extension = template_path.extension();
    let template_without_extension = if template_name.contains('.') {
        PathBuf::from(split_last(template_name, '.').unwrap().0)
    } else {
        PathBuf::from(template_name)
    };

    let custom_templates_dir = get_custom_templates_path(file);

    let mut found_template = None;
    if let Ok(read_dir) = custom_templates_dir.read_dir() {
        for entry in read_dir {
            match entry {
                Ok(entry) => {
                    let entry_path = entry.path();
                    if entry_path.is_file()
                        && eq_file_name(
                            &(entry_path.file_stem().unwrap()),
                            &template_without_extension,
                        )
                        && eq_file_extensions(
                            template_extension,
                            PathBuf::from(entry_path.file_stem().unwrap()).extension(),
                        )
                    {
                        found_template = Some(entry_path);
                    }
                }
                Err(_) => return Err(String::from("Couldn't read custom templates folder")),
            }
        }
    } else {
        return Err(String::from("Couldn't read custom templates folder"));
    }

    if let Some(found_template_path) = found_template {
        Ok(found_template_path)
    } else {
        Err(String::from("Couldn't found the provided template"))
    }
}

fn get_default_template(file: CreateableFileType) -> PathBuf {
    let mut default_template = get_out_dir();
    default_template.push_str("/templates/");

    match file {
        CreateableFileType::Page => default_template.push_str("page.tt"),
        CreateableFileType::ApiPage => default_template.push_str("api-page.tt"),
        CreateableFileType::Stylesheet => default_template.push_str("stylesheet.tt"),
        CreateableFileType::Component => default_template.push_str("component.tt"),
    }

    PathBuf::from(default_template)
}

fn get_custom_templates_path(file: CreateableFileType) -> PathBuf {
    let mut custom_template = PathBuf::from(format!("{}/{}/", NEXT_BUTLER_DIR, "templates/"));
    match file {
        CreateableFileType::Page => custom_template.push("pages/"),
        CreateableFileType::ApiPage => custom_template.push("api-pages/"),
        CreateableFileType::Stylesheet => custom_template.push("stylesheets/"),
        CreateableFileType::Component => custom_template.push("components/"),
    }

    custom_template
}
