use std::{fs, path::PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;
use indoc::indoc;

use crate::{helpers::{cli_helper::confirm_prompt, file_helper::{self, prepend_root_path, rm_file_by_stem}}, user_config::UserConfig, NextRouter};

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
            .arg(
                Arg::new("page-router")
                    .help("Removes the page located within the page router")
                    .conflicts_with("app-router")
                    .long("page-router")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("app-router")
                    .help("Removes the page located within the app router")
                    .long("app-router")
                    .action(ArgAction::SetTrue),
            ),
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
    let app_router_flag = args.get_flag("app-router");
    let page_router_flag = args.get_flag("page-router");
    let user_uses_page_router = UserConfig::get()?.get_page_config().page_router.unwrap_or(false);

    let router = if !app_router_flag && (page_router_flag || user_uses_page_router) {
        NextRouter::PageRouter
    } else {
        NextRouter::AppRouter
    };

    let page_arg = args.get_one::<String>("name").unwrap();

    let removal = match router {
        NextRouter::PageRouter => rm_page_from_page_router(page_arg),
        NextRouter::AppRouter => rm_page_from_app_router(page_arg)
    };

    if removal.is_ok() {
        println!("{}", "Page removed successfuly".green());
    }
    removal
}

fn rm_page_from_page_router(page_arg: &str) -> Result<(), String> {
    let router_dir_name = PathBuf::from("pages/");
    let mut router_path = prepend_root_path(router_dir_name)?;

    let confirmation = confirm_prompt(&format!("Do you want to delete the page '{}' and all it's components?", page_arg))?;
    if !confirmation {
        return Err(String::from("Operation cancelled."));
    }

    if page_arg == "/" {
        router_path.push("index");
        return file_helper::rm_file_by_stem(router_path)
    }

    router_path.push(page_arg);
    if !router_path.exists() {
        return Err(String::from("Page couldn't be found"));
    }

    rm_file_by_stem(router_path)
}

fn rm_page_from_app_router(page_arg: &str) -> Result<(), String> {
    let router_dir_name = PathBuf::from("app/");
    let mut router_path = prepend_root_path(router_dir_name)?;

    let confirmation = confirm_prompt(&format!("Do you want to delete the page '{}' and all it's components?", page_arg))?;
    if !confirmation {
        return Err(String::from("Operation cancelled."));
    }

    if page_arg == "/" {
        router_path.push("page");
        return file_helper::rm_file_by_stem(router_path)
    }

    router_path.push(page_arg);
    if !router_path.exists() {
        return Err(String::from("Page couldn't be found"));
    }

    fs::remove_dir_all(router_path).map_err(|err| err.to_string())
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
