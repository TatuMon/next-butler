use convert_case::{Case, Converter};
use std::fs;

use crate::{get_out_dir, CreateableFiles};

/// This pattern will be replaced by the name given to the file
const NAME_PATTERN: &str = "NNNN";

pub fn get_page_content(
    page_name: &str,
    is_api: bool,
    template: Option<&String>,
) -> Result<Vec<u8>, String> {
    let page_template: String;
    if is_api {
        page_template = get_template(template, CreateableFiles::ApiPage)?;
    } else {
        page_template = get_template(template, CreateableFiles::Page)?;
    }

    let read_attempt = fs::read_to_string(page_template);
    match read_attempt {
        Ok(content) => {
            let conv = Converter::new().to_case(Case::Pascal);
            let pascal_name = content.replace(NAME_PATTERN, &(conv.convert(page_name))[..]);
            Ok(pascal_name.as_bytes().to_owned())
        }
        Err(_) => Err(String::from("Couldn't read the page template")),
    }
}

pub fn get_component_content(
    component_name: &str,
    template: Option<&String>,
) -> Result<Vec<u8>, String> {
    let component_template = get_template(template, CreateableFiles::Component)?;

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
    let stylesheet_template = get_template(template, CreateableFiles::Component)?;

    match fs::read_to_string(stylesheet_template) {
        Ok(content) => {
            let conv = Converter::new().to_case(Case::Pascal);
            let final_content = content.replace(NAME_PATTERN, &(conv.convert(stylesheet_name))[..]);
            Ok(final_content.as_bytes().to_owned())
        }
        Err(_) => Err(String::from("Couldn't read the component template")),
    }
}

fn get_template(
    template_name: Option<&String>,
    file: CreateableFiles,
) -> Result<String, String> {
    let final_template;
    if let Some(custom_template) = template_name {
        final_template = get_custom_template(custom_template, file);
    } else {
        final_template = Ok(get_default_template(file));
    }

    final_template
}

fn get_custom_template(
    template_name: &String,
    file: CreateableFiles,
) -> Result<String, String> {
    Ok(String::from("hola"))
}

fn get_default_template(file: CreateableFiles) -> String {
    let mut default_template = get_out_dir();
    default_template.push_str("/templates/");

    match file {
        CreateableFiles::Page => default_template.push_str("page.tt"),
        CreateableFiles::ApiPage => default_template.push_str("api-page.tt"),
        CreateableFiles::Stylesheet => default_template.push_str("stylesheet.tt"),
        CreateableFiles::Component => default_template.push_str("component.tt"),
    }

    default_template
}
