pub mod commands;
pub mod constants;
pub mod helpers;
pub mod react_extension;
pub mod template;
pub mod user_config;

use std::{env, process};

use clap::Command;
use colored::Colorize;
use commands::{init_command, new_command};
use constants::{CRATE_NAME, CRATE_VERSION};

pub enum CreateableFileType {
    Page,
    ApiPage,
    Stylesheet,
    Component,
}

pub fn run() {
    let base_cmd = build_base_cmd();
    let app = new_command::set_subcommand(base_cmd);
    let app = init_command::set_subcommand(app);

    let base_cmd_args_matches = app.get_matches();
    let executed_cmd = match base_cmd_args_matches.subcommand() {
        Some(("new", cmd_args)) => new_command::exec_command(cmd_args),
        Some(("init", _)) => init_command::exec_command(),
        _ => Err(String::from("Unknown command")),
    };

    if let Err(err) = executed_cmd {
        eprintln!("{}", err.red());
        process::exit(1);
    }
}

/// The parent command.
/// Starting point of the tool.
pub fn build_base_cmd() -> Command {
    Command::new(CRATE_NAME)
        .about(
            "You can configure next-butler creating a file named \
                nextbutler.json inside your root folder. Go to the README \
                for more information.",
        )
        .version(CRATE_VERSION)
        .arg_required_else_help(true)
}

/// The path of the directory where the binary is compiled to
pub fn get_out_dir() -> String {
    env::var("OUT_DIR").unwrap()
}
