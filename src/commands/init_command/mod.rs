use std::{fs, path::PathBuf};

use clap::Command;

use crate::{user_config::UserConfig, helpers::file_helper};

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

    fs::create_dir_all(nextbutler_path.to_owned())
        .map_err(|err| format!("Error creating nextbutler folder: {}", err.to_string()))?;

    let user_config_path = nextbutler_path.join("nextbutler.json");
    let default_user_config = serde_json::to_vec_pretty(&UserConfig::get_default())
        .map_err(|err| format!("Error building the default configuration file: {}", err.to_string()))?;

    file_helper::create(&user_config_path, default_user_config)?;

    Ok(())
}
