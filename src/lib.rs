pub mod commands;
pub mod constants;
pub mod helpers;

use clap::Command;
use commands::new_command;
use constants::{CRATE_NAME, CRATE_VERSION};

pub fn run(app: Command) {
    let app = new_command::set_subcommand(app);
    // let app = x_command::set_subcommand(app);
    // let app = y_command::set_subcommand(app);

    let app_m = app.get_matches();
    match app_m.subcommand() {
        Some(("new", new_args)) => new_command::exec_command(new_args),
        _ => {
            println!("Unknown command")
        }
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
