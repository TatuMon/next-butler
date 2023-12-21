use clap::{Arg, ArgAction, ArgMatches, Command};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use crate::{helpers::file_helper, user_config::UserNewPageConfig};

use self::final_new_page_config::FinalNewPageConfig;

pub mod final_new_page_config;

pub enum PageExtension {
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

impl From<&PageExtension> for &'static str {
    fn from(e: &PageExtension) -> Self {
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
            _ => PageExtension::Js,
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
            _ => PageExtension::Js,
        }
    }
}

impl PageExtension {
    fn guess(js_flag: bool, tsx_flag: bool, ts_flag: bool, user_new_page_cfg: Option<UserNewPageConfig>) -> Self {
        if js_flag {
            Self::Js
        } else if tsx_flag {
            Self::Tsx
        } else if ts_flag {
            Self::Ts
        } else if user_new_page_cfg.is_none() {
            Self::Jsx
        } else if let Some(user_cfg) = user_new_page_cfg {
            user_cfg.guess_extension()
        } else {
            Self::Jsx
        }
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
                    .conflicts_with("js")
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
                Arg::new("jsx")
                    .help("Define if the file should have the .tsx extension")
                    .long("jsx")
                    .alias("tsx")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("template")
                    .help("The name of your custom template")
                    .long("template"),
            )
            .arg(
                Arg::new("page-router")
                    .help("Create the page based on the page router")
                    .conflicts_with("app-router")
                    .long("page-router")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("app-router")
                    .help("Create the page based on the app router")
                    .long("app-router")
                    .action(ArgAction::SetTrue),
            ),
    )
}

/// Creates a new page based on the given arguments and the configuration file
pub fn exec_command(cmd_args: &ArgMatches) -> Result<(), String> {
    let page_args = FinalNewPageConfig::new(cmd_args)?;

    file_helper::create(&page_args.page_final_path, page_args.template.content)?;
    Ok(())
}

/// Returns the final path of the page (Inside src/pages/ or /pages,
/// depending on the project), with the correct file extension
fn get_page_final_path(
    page_path: PathBuf,
    extension: &PageExtension,
    use_page_router: bool,
) -> Result<PathBuf, String> {
    let extension_str: &str = extension.into();
    let mut final_page_path = setup_page_path(page_path, use_page_router)?;

    if final_page_path.set_extension(extension_str) {
        Ok(final_page_path)
    } else {
        Err(String::from("Error setting the extension"))
    }
}

/// Set the parents to page_path, based on the correct router (app or page router)
fn setup_page_path(page_path: PathBuf, use_page_router: bool) -> Result<PathBuf, String> {
    // Remove / prefix, so the 'push' function doesn't overwrite the path
    let page_relative_path = page_path
        .strip_prefix("/")
        .unwrap_or(page_path.as_path())
        .to_path_buf();

    // Base path of the new page
    let mut path_prefix = PathBuf::new();

    if file_helper::is_src_present()? {
        path_prefix.push("src/");
    }

    if use_page_router {
        path_prefix.push("pages/");

        Ok(path_prefix.join(page_relative_path))
    } else {
        path_prefix.push("app/");
        if let Some(page_name) = page_relative_path.file_stem() {
            path_prefix.push(format!("{}/page", page_name.to_string_lossy()));
        } else {
            return Err(String::from("Page name not provided."));
        }

        Ok(path_prefix)
    }
}

/// Returns true if the name starts with
/// "api/"
fn is_api(page_name: &Path) -> bool {
    match page_name.strip_prefix(MAIN_SEPARATOR_STR) {
        Ok(trimmed) => trimmed.starts_with("api/") || trimmed.starts_with("api\\"),
        Err(_) => page_name.starts_with("api/") || page_name.starts_with("api\\"),
    }
}
