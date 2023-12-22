use std::{fs, path::PathBuf};

use clap::Command;

use crate::{helpers::file_helper, user_config::UserConfig, template::Template};

/// Sets the subcommand and the corresponding arguments
pub fn set_subcommand(app: Command) -> Command {
    // Set the subcommand 'new'
    let new_subcommand =
        Command::new("init").about("Creates the tool folder and a basic configuration file.");

    app.subcommand(new_subcommand)
}

/// Executes the command
pub fn exec_command() -> Result<(), String> {
    println!("Creating basic configuration...");

    let nextbutler_path = PathBuf::from("nextbutler/");
    fs::create_dir_all(nextbutler_path.clone())
        .map_err(|err| format!("Error creating nextbutler folder: {}", err.to_string()))?;

    // Create configuration file
    let user_config_path = nextbutler_path.join("nextbutler.json");
    file_helper::create(&user_config_path, UserConfig::get_default_as_vec()?)?;

    // Create page templates folder
    Template::create_default_page_template(nextbutler_path.join("templates/page/"))?;
    Template::create_default_component_template(nextbutler_path.join("templates/components/"))?;
    Template::create_default_stylesheet_template(nextbutler_path.join("templates/styles/"))?;

    Ok(())
}
