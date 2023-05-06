use clap::{Arg, ArgAction, ArgMatches, Command};
use path_clean::clean;
use std::{
    env, fs,
    path::{PathBuf, MAIN_SEPARATOR_STR},
};

use crate::helpers::file_helper;

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
                    .help("Define if the file should end with .ts")
                    .long("ts")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("jsx")
                    .help(
                        "Define if the file should end with .jsx \
                              (or .tsx if --ts is set)",
                    )
                    .long("jsx")
                    .action(ArgAction::SetTrue),
            ),
    )
}

pub fn exec_command(page_args: &ArgMatches) -> Result<(), String> {
    let jsx_flag = page_args.get_flag("jsx");
    let ts_flag = page_args.get_flag("ts");
    let page_path = get_page_final_path(page_args.get_one("page_path"))?;
    let is_api = is_api(&page_path);
    let page_content = read_page_template(is_api)?;

    file_helper::create(&page_path, &page_content)?;
    Ok(())
}

/// Returns the final path of the page.
/// (Inside src/pages/ or /pages, depending on the project)
fn get_page_final_path(page_path: Option<&String>) -> Result<PathBuf, String> {
    let valid_page_str: &String;
    match page_path {
        None => return Err(String::from("Invalid page path")),
        Some(page_str) => valid_page_str = page_str,
    }

    let mut final_path = file_helper::validate_filepath(valid_page_str)?;

    let mut path_prefix = String::new();
    if file_helper::is_src_present()? {
         path_prefix.push_str("/src/");
    }

    path_prefix.push_str("/pages/");

    final_path = final_path.join(path_prefix).ps;

    Ok(final_path)
}

/// Returns true if the name starts with
/// "api/"
fn is_api(page_name: &PathBuf) -> bool {
    match page_name.as_path().strip_prefix(MAIN_SEPARATOR_STR) {
        Ok(trimmed) => trimmed.starts_with("api/") || trimmed.starts_with("api\\"),
        Err(_) => page_name.starts_with("api/") || page_name.starts_with("api\\"),
    }
}

/// Attempts to read the template for a page file
fn read_page_template(is_api: bool) -> Result<Vec<u8>, String> {
    let exe_path = env::current_exe();
    if let Err(_) = exe_path {
        return Err(String::from("Couldn't read the page template"));
    } else if let Ok(path) = exe_path {
        let mut exe_dir = path.parent().unwrap_or(&path.as_path()).to_path_buf();
        if is_api {
            exe_dir.push("templates/api-page.tt");
        } else {
            exe_dir.push("templates/page.tt");
        }

        let read_attempt = fs::read(path);
        return match read_attempt {
            Ok(content) => Ok(content),
            Err(_) => Err(String::from("Couldn't read the page template")),
        };
    } else {
        Err(String::from("Couldn't read the page template"))
    }
}
