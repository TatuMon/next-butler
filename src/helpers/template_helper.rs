use convert_case::{Case, Converter};
use std::fs;

use crate::get_out_dir;

/// This pattern will be replaced by the name given to the file
const NAME_PATTERN: &str = "NNNN";

pub fn get_page_content(page_name: &str, is_api: bool) -> Result<Vec<u8>, String> {
    let mut page_template = get_out_dir();
    if is_api {
        page_template.push_str("/templates/api-page.tt");
    } else {
        page_template.push_str("/templates/page.tt");
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

pub fn get_component_content(component_name: &str) -> Result<Vec<u8>, String> {
    let mut component_template = get_out_dir();
    component_template.push_str("/templates/component.tt");

    match fs::read_to_string(component_template) {
        Ok(content) => {
            let conv = Converter::new().to_case(Case::Pascal);
            let final_content = content.replace(NAME_PATTERN, &(conv.convert(component_name))[..]);
            Ok(final_content.as_bytes().to_owned())
        }
        Err(_) => Err(String::from("Couldn't read the component template")),
    }
}

pub fn get_stylesheet_content(stylesheet_name: &str) -> Result<Vec<u8>, String> {
    let mut stylesheet_template = get_out_dir();
    stylesheet_template.push_str("/templates/stylesheet.tt");

    match fs::read_to_string(stylesheet_template) {
        Ok(content) => {
            let conv = Converter::new().to_case(Case::Pascal);
            let final_content = content.replace(NAME_PATTERN, &(conv.convert(stylesheet_name))[..]);
            Ok(final_content.as_bytes().to_owned())
        }
        Err(_) => Err(String::from("Couldn't read the component template")),
    }
}
