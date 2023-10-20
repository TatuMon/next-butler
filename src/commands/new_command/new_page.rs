use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR};

use crate::helpers::{
    file_helper::{self, get_name_or_err},
    template_helper::get_page_content,
};

struct NewPageCommandArgs {
    page_path: PathBuf,
    jsx_flag: bool,
    ts_flag: bool,
    template: Option<PathBuf>,
    is_api: bool,
}

impl NewPageCommandArgs {
    fn new(page_args: &ArgMatches) -> Result<Self, String> {
        let page_path = PathBuf::from(page_args.get_one::<String>("page_path").unwrap());
        let jsx_flag;
        let ts_flag;
        if page_args.get_flag("tsx") {
            jsx_flag = true;
            ts_flag = true;
        } else {
            jsx_flag = page_args.get_flag("jsx");
            ts_flag = page_args.get_flag("ts");
        }

        let template = page_args
            .get_one::<String>("template")
            .map(PathBuf::from);

        let is_api = is_api(&page_path);

        Ok(NewPageCommandArgs {
            page_path,
            jsx_flag,
            ts_flag,
            is_api,
            template,
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
                Arg::new("jsx")
                    .help(
                        "Define if the file should have the .jsx extension\
                              (or .tsx if --ts is set)",
                    )
                    .long("jsx")
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
    let page_args = NewPageCommandArgs::new(cmd_args)?;

    let page_final_path = get_page_final_path(
        page_args.page_path.to_owned(),
        page_args.jsx_flag,
        page_args.ts_flag,
    )?;
    let page_name = get_name_or_err(&page_final_path)?;
    let page_content = get_page_content(page_name, page_args.is_api, page_args.template)?;

    file_helper::create(&page_final_path, page_content)?;
    Ok(())
}

/// Returns the final path of the page (Inside src/pages/ or /pages,
/// depending on the project), with the correct file extension, depending on
/// the configuration and the provided flags
fn get_page_final_path(page_path: PathBuf, is_jsx: bool, is_ts: bool) -> Result<PathBuf, String> {
    let page_path = page_add_file_ext(page_path, is_jsx, is_ts)?;
    let page_path = page_add_path_prefix(page_path)?;

    Ok(page_path)
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

fn page_add_file_ext(mut page_path: PathBuf, is_jsx: bool, is_ts: bool) -> Result<PathBuf, String> {
    let ext_modified: bool;
    if is_jsx {
        if is_ts {
            ext_modified = page_path.set_extension("tsx")
        } else {
            ext_modified = page_path.set_extension("jsx")
        }
    } else if is_ts {
        ext_modified = page_path.set_extension("ts")
    } else {
        ext_modified = page_path.set_extension("js")
    }

    if ext_modified {
        Ok(page_path)
    } else {
        Err(String::from("Couldn't set file extension"))
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
