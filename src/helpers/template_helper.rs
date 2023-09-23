use std::{fs, io::Error, env};
use convert_case::{Case, Converter};

/// This pattern will be replaced by the name given to the file
const NAME_PATTERN: &str = "NNNN";

pub fn get_page_content(page_name: &str, is_api: bool) -> Result<Vec<u8>, String> {
    let exe_path = env::current_exe();
    if let Err(_) = exe_path {
        return Err(String::from("Error finding the page template"));
    } else if let Ok(path) = exe_path {
        let mut exe_dir = path.parent().unwrap_or(&path.as_path()).to_path_buf();
        if is_api {
            exe_dir.push("templates/api-page.tt");
        } else {
            exe_dir.push("templates/page.tt");
        }

        let read_attempt = fs::read_to_string(path);
        match read_attempt {
            Ok(content) => {
                let conv = Converter::new().to_case(Case::Pascal);
                let pascal_name = content.replace(
                    NAME_PATTERN,
                    &(conv.convert(page_name))[..]
                );
                Ok(pascal_name.as_bytes().to_owned())
            },
            Err(_) => Err(String::from("Couldn't read the page template")),
        }
    } else {
        Err(String::from("Couldn't find the page template"))
    }
}

pub fn get_api_page_content() -> Result<String, Error> {
    Ok(fs::read_to_string("templates/api-page.tt")?)
}

pub fn get_component_content(component_name: &str) -> Result<String, Error> {
    let component_content = fs::read_to_string("templates/components.tt")?;
    let component_content = component_content
                                .replace(NAME_PATTERN, component_name);

    Ok(component_content)
}
