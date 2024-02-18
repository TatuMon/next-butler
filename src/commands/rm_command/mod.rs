use std::{fs, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use indoc::indoc;

use crate::{user_config::UserConfig, helpers::file_helper::prepend_root_path};

pub fn set_subcommand(app: Command) -> Command {
    // Set the subcommand 'rm'
    let rm_subcommand = Command::new("rm").about("Remove a page, component or stylesheet.");

    let rm_subcommand =
        rm_subcommand.subcommand(Command::new("page")
            .about("")
            .arg(
                Arg::new("name").required(true).help(indoc! {"
                    Search for this page route in the appropiate router folder.
                    Read the custom config documentation for more info: https://github.com/TatuMon/next-butler?tab=readme-ov-file#config-file.
                "}),
            )
        );
    let rm_subcommand =
        rm_subcommand.subcommand(Command::new("component")
            .about("Remove a file or folder inside the components folder.")
            .arg(
                Arg::new("name").required(true).help(indoc! {"
                    Search for this file (or folder) in the folder defined in the new component configuration.
                    Read the custom config documentation for more info: https://github.com/TatuMon/next-butler?tab=readme-ov-file#config-file.
                "}),
            )
        );
    let rm_subcommand =
        rm_subcommand.subcommand(Command::new("style")
            .about("Remove a file or folder inside the stylesheets folder.")
            .arg(
                Arg::new("name").required(true).help(indoc! {"
                    Search for this file (or folder) in the folder defined in the new stylesheet configuration.
                    Read the custom config documentation for more info: https://github.com/TatuMon/next-butler?tab=readme-ov-file#config-file.
                "}),
            )
        );

    // Attaches the subcommand 'rm' to the main command
    app.subcommand(rm_subcommand)
}

/// Executes the command
pub fn exec_command(cmd_args: &ArgMatches) -> Result<(), String> {
    let subcmd = cmd_args.subcommand();
    match subcmd {
        Some(("page", new_page_cmd_args)) => rm_page(new_page_cmd_args),
        Some(("component", new_comp_cmd_args)) => rm_component(new_comp_cmd_args),
        Some(("style", new_style_cmd_args)) => rm_stylesheet(new_style_cmd_args),
        _ => Err(String::from("Unknown command")),
    }
}

/// Pages are removed by removing the entire dir with it's name within the
/// appropiate router directory. So it's layout and other related files gets
/// deleted too, except for the home page (read below).
///
/// In case of the page having child pages (like when trying to delete
/// "/players" while it has "/profiles"), these get deleted as well.
///
/// When removing the home page (by removing "/", "index" in the page router or
/// "page" in the app router), only the page component gets deleted, while the layout
/// and other related files are kept.
fn rm_page(args: &ArgMatches) -> Result<(), String> {
    let uses_page_router = UserConfig::get()?.get_page_config().page_router.unwrap_or(false);
    let router_dir = if uses_page_router {
        PathBuf::from("pages/")
    } else {
        PathBuf::from("app/")
    };
    let mut router_path = prepend_root_path(PathBuf::from(router_dir));
    let page_arg = args.get_one::<String>("name").unwrap();




    Ok(())
}

fn rm_component(args: &ArgMatches) -> Result<(), String> {
    let name_arg = args.get_one::<String>("name").unwrap();
    let mut comps_folder = prepend_root_path(PathBuf::from(
        UserConfig::get()?
            .get_component_config()
            .folder
            .unwrap_or(String::from("components")),
    ))?;
    comps_folder.push(name_arg);


    if !comps_folder.exists() {
        Err(String::from("Target component doesn't exist"))
    } else if comps_folder.is_file() {
        fs::remove_file(comps_folder).map_err(|_| String::from("Error deleting component's file"))
    } else {
        fs::remove_dir_all(comps_folder)
            .map_err(|_| String::from("Error deleting component's folder"))
    }?;

    println!("{}", "Component successfully removed".green());
    Ok(())
}

fn rm_stylesheet(args: &ArgMatches) -> Result<(), String> {
    let name_arg = args.get_one::<String>("name").unwrap();
    let mut styles_folder = prepend_root_path(PathBuf::from(
        UserConfig::get()?
            .get_style_config()
            .folder
            .unwrap_or(String::from("styles")),
    ))?;
    styles_folder.push(name_arg);

    if !styles_folder.exists() {
        Err(String::from("Target stylesheet doesn't exist"))
    } else if styles_folder.is_file() {
        fs::remove_file(styles_folder).map_err(|_| String::from("Error deleting stylesheet's file"))
    } else {
        fs::remove_dir_all(styles_folder)
            .map_err(|_| String::from("Error deleting stylesheet's folder"))
    }?;

    println!("{}", "Stylesheet successfully removed".green());
    Ok(())
}
