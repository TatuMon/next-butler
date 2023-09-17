use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::{PathBuf, MAIN_SEPARATOR_STR};

use crate::helpers::{
    file_helper::{self, get_name_or_err},
    template_helper::get_page_content,
};

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
            ),
    )
}

/// Creates a new page based on the given arguments and the configuration file
pub fn exec_command(page_args: &ArgMatches) -> Result<(), String> {
    // Get command parameters
    let page_path = PathBuf::from(page_args.get_one::<String>("page_path").unwrap());
    let jsx_flag = page_args.get_flag("jsx");
    let ts_flag = page_args.get_flag("ts");

    let is_api = is_api(&page_path);
    let page_final_path = get_page_final_path(page_path, jsx_flag, ts_flag)?;
    let page_name = get_name_or_err(&page_final_path)?;
    let page_content = get_page_content(page_name, is_api)?;

    file_helper::create(&page_final_path, page_content)?;
    Ok(())
}

/// Returns the final path of the page (Inside src/pages/ or /pages,
/// depending on the project), with the correct file extension, depending on
/// the configuration and the provided flags
fn get_page_final_path(page_path: PathBuf, is_jsx: bool, is_ts: bool) -> Result<PathBuf, String> {
    let page_path = page_add_path_prefix(page_path)?;
    let page_path = page_add_file_ext(page_path, is_jsx, is_ts)?;

    Ok(page_path)
}

fn page_add_path_prefix(page_path: PathBuf) -> Result<PathBuf, String> {
    let final_path = file_helper::validate_filepath(page_path)?;

    // Base path of the new page
    let mut path_prefix = String::new();

    if file_helper::is_src_present()? {
        path_prefix.push_str("/src/");
    }

    path_prefix.push_str("/pages/");

    Ok(final_path.join(path_prefix))
}

fn page_add_file_ext(mut page_path: PathBuf, is_jsx: bool, is_ts: bool) -> Result<PathBuf, String> {
    let ext_modified;
    if is_jsx {
        if is_ts {
            ext_modified = page_path.set_extension(".tsx")
        } else {
            ext_modified = page_path.set_extension(".jsx")
        }
    } else {
        if is_ts {
            ext_modified = page_path.set_extension(".ts")
        } else {
            ext_modified = page_path.set_extension(".js")
        }
    }

    if ext_modified {
        Ok(page_path)
    } else {
        Err(String::from("Couldn't set file extension"))
    }
}

/// Returns true if the name starts with
/// "api/"
fn is_api(page_name: &PathBuf) -> bool {
    match page_name.as_path().strip_prefix(MAIN_SEPARATOR_STR) {
        Ok(trimmed) => trimmed.starts_with("api/") || trimmed.starts_with("api\\"),
        Err(_) => page_name.starts_with("api/") || page_name.starts_with("api\\"),
    }
}
