use std::{fs, io::Error};

/// This pattern will be replaced by the name given to the file
const NAME_PATTERN: &str = "NNNN";

pub fn get_page_content(page_name: &str) -> Result<String, Error> {
    let page_content = fs::read_to_string("templates/page.txt")?;
    let page_content = page_content.replace(NAME_PATTERN, page_name);

    Ok(page_content)
}

pub fn get_api_page_content() -> Result<String, Error> {
    Ok(fs::read_to_string("templates/api-page.txt")?)
}

pub fn get_component_content(component_name: &str) -> Result<String, Error> {
    let component_content = fs::read_to_string("templates/components.txt")?;
    let component_content = component_content
                                .replace(NAME_PATTERN, component_name);

    Ok(component_content)
}
