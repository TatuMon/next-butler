use std::{fs, path::PathBuf};

use clap::Command;

use crate::{helpers::file_helper, template::{create_pages_templates, create_components_templates, create_stylesheets_templates}, user_config::UserConfig};
use colored::Colorize;

/// Sets the subcommand and the corresponding arguments
pub fn set_subcommand(app: Command) -> Command {
    // Set the subcommand 'new'
    let new_subcommand = Command::new("init")
        .about("Creates the tool's configuration structure")
        .after_help(
            "This command does the following: \n\
                    - Creates the tool's directory (nextbutler) inside the root dir \n\
                    - Creates the configuration file (nextbutler/nextbutler.json) \n\
                    - Creates the default templates as custom ones, as examples",
        );

    app.subcommand(new_subcommand)
}

/// Executes the command
pub fn exec_command() -> Result<(), String> {
    println!("Creating basic configuration...");

    let nextbutler_path = PathBuf::from("nextbutler/");
    fs::create_dir_all(nextbutler_path.clone())
        .map_err(|err| format!("Error creating nextbutler directory: {}", err))?;

    // Create configuration file
    let user_config_path = nextbutler_path.join("nextbutler.json");
    file_helper::create(&user_config_path, UserConfig::get_default_as_vec()?)
        .map_err(|err| format!("Error creating configuration file: {}", err))?;

    // Create page templates folder
    println!("Creating templates...");
    create_pages_templates(nextbutler_path.join("templates/pages/"))?;
    create_components_templates(nextbutler_path.join("templates/components/"))?;
    create_stylesheets_templates(nextbutler_path.join("templates/styles/"))?;

    println!("{}", "Configuration structure created successfuly".green());

    Ok(())
}
