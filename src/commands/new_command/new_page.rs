use clap::{Arg, ArgAction, ArgMatches, Command};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use crate::{
    helpers::{
        file_helper::{self, get_name_or_err},
        template_helper::{get_page_content, get_template, Template},
    },
    CreateableFileType,
};

enum PageExtension {
    Jsx,
    Tsx,
    Js,
    Ts,
}

impl From<PageExtension> for &'static str {
    fn from(e: PageExtension) -> Self {
        match e {
            PageExtension::Jsx => "jsx",
            PageExtension::Tsx => "tsx",
            PageExtension::Js => "js",
            PageExtension::Ts => "ts",
        }
    }
}

impl From<&str> for PageExtension {
    fn from(e: &str) -> Self {
        match e {
            "jsx" => PageExtension::Jsx,
            "tsx" => PageExtension::Tsx,
            "js" => PageExtension::Js,
            "ts" => PageExtension::Ts,
            _ => PageExtension::Jsx,
        }
    }
}

impl From<&OsStr> for PageExtension {
    fn from(e: &OsStr) -> Self {
        match e.to_string_lossy().to_string().as_str() {
            "jsx" => PageExtension::Jsx,
            "tsx" => PageExtension::Tsx,
            "js" => PageExtension::Js,
            "ts" => PageExtension::Ts,
            _ => PageExtension::Jsx,
        }
    }
}

impl PageExtension {
    fn guess(js_flag: bool, tsx_flag: bool, ts_flag: bool, template: &Template) -> Self {
        if template.is_custom && template.path.extension().is_some() {
            template.path.extension().unwrap().into()
        } else if js_flag {
            Self::Js
        } else if tsx_flag {
            Self::Tsx
        } else if ts_flag {
            Self::Ts
        } else {
            Self::Jsx
        }
    }
}

struct NewPageConfig {
    /// Where the new page will be located
    page_final_path: PathBuf,
    /// Final content of the page
    page_content: Vec<u8>,
}

impl NewPageConfig {
    fn new(page_args: &ArgMatches) -> Result<Self, String> {
        let path_arg = PathBuf::from(page_args.get_one::<String>("page_path").unwrap());
        let page_type = if is_api(&path_arg) {
            CreateableFileType::ApiPage
        } else {
            CreateableFileType::Page
        };

        let template = get_template(page_args.get_one::<String>("template"), page_type)?;
        let page_extension = PageExtension::guess(
            page_args.get_flag("js"),
            page_args.get_flag("tsx"),
            page_args.get_flag("ts"),
            &template,
        );

        let coso: &str = page_extension.into();
        return Err(template.path.to_string_lossy().to_string());

        let page_final_path = get_page_final_path(path_arg.to_owned(), page_extension)?;
        let page_name = get_name_or_err(&page_final_path)?;
        let page_content = get_page_content(page_name, template)?;

        Ok(Self {
            page_final_path,
            page_content,
        })
    }
}

/// Sets the new page subcommand
pub fn set_subcommand(app: Command) -> Command {
    app.subcommand(
        Command::new("page")
            .about("Create a new page file, inside /pages/")
            .arg(
                Arg::new("page_path")
                    .required(true)
                    .help("The path of the page file."),
            )
            .arg(
                Arg::new("ts")
                    .help("Define if the file is a typescript one")
                    .long("ts")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("js")
                    .help(
                        "Define if the file should have the .js extension\
                              (.jsx is the default for pages)",
                    )
                    .long("js")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("tsx")
                    .help("Define if the file should have the .tsx extension")
                    .long("tsx")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("template")
                    .help("The name of your custom template")
                    .long("template"),
            ),
    )
}

/// Creates a new page based on the given arguments and the configuration file
pub fn exec_command(cmd_args: &ArgMatches) -> Result<(), String> {
    let page_args = NewPageConfig::new(cmd_args)?;

    file_helper::create(&page_args.page_final_path, page_args.page_content)?;
    Ok(())
}

/// Returns the final path of the page (Inside src/pages/ or /pages,
/// depending on the project), with the correct file extension
fn get_page_final_path(
    mut page_path: PathBuf,
    extension: PageExtension,
) -> Result<PathBuf, String> {
    let extension_str: &str = extension.into();
    page_path.set_extension(extension_str);

    page_add_path_prefix(page_path)
}

fn page_add_path_prefix(page_path: PathBuf) -> Result<PathBuf, String> {
    // Remove / prefix
    let page_relative_path = page_path
        .strip_prefix("/")
        .unwrap_or(page_path.as_path())
        .to_path_buf();

    // Base path of the new page
    let mut path_prefix = PathBuf::new();

    if file_helper::is_src_present()? {
        path_prefix.push("src/");
    }
    path_prefix.push("pages/");

    let final_path = path_prefix.join(page_relative_path);
    Ok(final_path)
}

/// Returns true if the name starts with
/// "api/"
fn is_api(page_name: &Path) -> bool {
    match page_name.strip_prefix(MAIN_SEPARATOR_STR) {
        Ok(trimmed) => trimmed.starts_with("api/") || trimmed.starts_with("api\\"),
        Err(_) => page_name.starts_with("api/") || page_name.starts_with("api\\"),
    }
}
