use std::{
    collections::BTreeMap,
    fs::{self, File},
    path::{Path, PathBuf},
};

use handlebars::Handlebars;

use crate::{
    constants::NEXT_BUTLER_DIR,
    helpers::file_helper::{self, get_file_stem_occurrences},
    CreateableFileType,
};

use self::default_templates::{
    DEFAULT_API_PAGE_TEMPLATE, DEFAULT_COMPONENT_TEMPLATE, DEFAULT_PAGE_TEMPLATE,
    DEFAULT_STYLESHEET_TEMPLATE,
};

pub mod default_templates;
pub mod template_variables;

pub enum Template<'a> {
    // Content of the template, used only when loading default templates
    Str(&'a str),
    // Path to the template, used when loading custom templates
    Path(PathBuf),
}

pub fn get_custom_template<'a>(
    template_name: &str,
    file_type_to_create: &CreateableFileType,
) -> Result<Template<'a>, String> {
    let template_arg_path = PathBuf::from(template_name);
    let custom_templates_dir = get_custom_templates_dir(file_type_to_create)?;

    // If the user specified the custom template extension, directly search the
    // file
    if let Some(tmpl_arg_extension) = template_arg_path.extension() {
        let mut tmpl_arg_extension = tmpl_arg_extension.to_owned();
        tmpl_arg_extension.push(".hbs");

        let mut tmpl_full_path = custom_templates_dir.join(template_arg_path);
        tmpl_full_path.set_extension(tmpl_arg_extension);
        if tmpl_full_path.is_file() {
            return Ok(Template::Path(tmpl_full_path));
        } else {
            return Err(String::from(
                "Couldn't find a template with the given extension",
            ));
        }
    }

    // If not, search the templates with the given name.
    // It will return an error if none or more than one are found
    if let Some(arg_tmpl_stem) = template_arg_path.file_stem() {
        let found_tmpls = get_file_stem_occurrences(arg_tmpl_stem, &custom_templates_dir)?;

        if found_tmpls.is_empty() {
            Err(String::from("Couldn't find the provided template"))
        } else if found_tmpls.len() > 1 {
            return Err(String::from(
                "Found multiple templates with the same name. Please specify the extension",
            ));
        } else {
            return found_tmpls
                .first()
                .map(|p| Template::Path(p.to_owned()))
                .ok_or(String::from("Couldn't find the provided template"));
        }
    } else {
        Err(String::from("Wrong template name"))
    }
}

pub fn get_default_template<'a>(file: &CreateableFileType) -> Template<'a> {
    let template_content = match file {
        CreateableFileType::Page => DEFAULT_PAGE_TEMPLATE,
        CreateableFileType::ApiPage => DEFAULT_API_PAGE_TEMPLATE,
        CreateableFileType::Stylesheet => DEFAULT_STYLESHEET_TEMPLATE,
        CreateableFileType::Component => DEFAULT_COMPONENT_TEMPLATE,
    };

    Template::Str(template_content)
}

fn get_custom_templates_dir(file: &CreateableFileType) -> Result<PathBuf, String> {
    let mut custom_templates_path = PathBuf::from(format!("{}/{}/", NEXT_BUTLER_DIR, "templates/"));
    match file {
        CreateableFileType::Page => custom_templates_path.push("pages/"),
        CreateableFileType::ApiPage => custom_templates_path.push("api-pages/"),
        CreateableFileType::Stylesheet => custom_templates_path.push("stylesheets/"),
        CreateableFileType::Component => custom_templates_path.push("components/"),
    }

    if custom_templates_path.exists() {
        Ok(custom_templates_path)
    } else {
        Err(String::from("Custom templates directory is not defined"))
    }
}

pub fn create_pages_templates<P>(pages_templates_dir: P) -> Result<(), String>
where
    P: AsRef<Path>,
{
    fs::create_dir_all(pages_templates_dir.as_ref())
        .map_err(|err| format!("Error creating page templates folder: {}", err))
        .and_then(|()| {
            file_helper::create(
                &PathBuf::from(pages_templates_dir.as_ref()).join("default.jsx.hbs"),
                DEFAULT_PAGE_TEMPLATE.as_bytes().to_vec(),
            )
        })
        .and_then(|()| {
            file_helper::create(
                &PathBuf::from(pages_templates_dir.as_ref()).join("api/default.js.hbs"),
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
                &PathBuf::from(components_tempaltes_dir.as_ref()).join("default.jsx.hbs"),
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
                &PathBuf::from(stylesheets_templates_dir.as_ref()).join("default.css.hbs"),
                DEFAULT_STYLESHEET_TEMPLATE.as_bytes().to_vec(),
            )
        })?;

    Ok(())
}

pub fn create_from_template(new_file_path: &Path, template: Template, template_vars: &BTreeMap<String, String>) -> Result<(), String> {
    let mut handlebars = Handlebars::new();
    match template {
        Template::Str(tmpl_content) => {
            handlebars
                .register_template_string("template", tmpl_content)
                .map_err(|err| err.to_string())?;
        }
        Template::Path(tmpl_path) => {
            handlebars
                .register_template_file("template", tmpl_path)
                .map_err(|err| err.to_string())?;
        }
    }

    if let Some(parent_dir) = new_file_path.parent() {
        fs::create_dir_all(parent_dir).map_err(|err| format!("Error creating file: {}", err))?;
    }
    let file = File::create_new(new_file_path).map_err(|err| format!("Error creating file: {}", err))?;

    handlebars
        .render_to_write("template", &template_vars, &file)
        .map_err(|err| err.to_string())
}
