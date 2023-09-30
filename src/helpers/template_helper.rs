use convert_case::{Case, Converter};
use std::{fs, path::PathBuf};

use crate::{
    constants::NEXT_BUTLER_DIR, get_out_dir, helpers::file_helper::eq_file_name, CreateableFiles,
};

use super::file_helper::eq_file_extensions;

/// This pattern will be replaced by the name given to the file
const NAME_PATTERN: &str = "NNNN";

pub fn get_page_content(
    page_name: &str,
    is_api: bool,
    template: Option<&String>,
) -> Result<Vec<u8>, String> {
    let page_template: PathBuf;
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

fn get_template(template_name: Option<&String>, file: CreateableFiles) -> Result<PathBuf, String> {
    let final_template;
    if let Some(custom_template) = template_name {
        final_template = get_custom_template(custom_template, file);
    } else {
        final_template = Ok(get_default_template(file));
    }

    final_template
}

fn get_custom_template(template_name: &String, file: CreateableFiles) -> Result<PathBuf, String> {
    let template_path = PathBuf::from(template_name);

    let template_extension = template_path.extension();
    let template_without_extension;
    if template_name.contains(".") {
        template_without_extension = PathBuf::from(template_name.rsplitn(1, ".").next().unwrap());
    } else {
        template_without_extension = PathBuf::from(template_name);
    }

    let custom_templates_dir = get_custom_templates_path(file);

    let mut found_template = None;
    if let Ok(read_dir) = custom_templates_dir.read_dir() {
        for entry in read_dir {
            match entry {
                Ok(entry) => {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        if eq_file_name(
                            &(entry_path.file_stem().unwrap()),
                            &template_without_extension,
                        ) && eq_file_extensions(
                            template_extension,
                            PathBuf::from(entry_path.file_stem().unwrap()).extension(),
                        ) {
                            found_template = Some(entry_path);
                        }
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

fn get_default_template(file: CreateableFiles) -> PathBuf {
    let mut default_template = get_out_dir();
    default_template.push_str("/templates/");

    match file {
        CreateableFiles::Page => default_template.push_str("page.tt"),
        CreateableFiles::ApiPage => default_template.push_str("api-page.tt"),
        CreateableFiles::Stylesheet => default_template.push_str("stylesheet.tt"),
        CreateableFiles::Component => default_template.push_str("component.tt"),
    }

    PathBuf::from(default_template)
}

fn get_custom_templates_path(file: CreateableFiles) -> PathBuf {
    let mut custom_template = PathBuf::from(format!("{}/{}/", NEXT_BUTLER_DIR, "templates/"));
    match file {
        CreateableFiles::Page => custom_template.push("pages/"),
        CreateableFiles::ApiPage => custom_template.push("api-pages/"),
        CreateableFiles::Stylesheet => custom_template.push("stylesheets/"),
        CreateableFiles::Component => custom_template.push("components/"),
    }

    custom_template
}
