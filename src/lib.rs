pub mod commands;
pub mod constants;

use clap::Command;
use commands::new_command;

pub fn run(app: Command) {
    let app = new_command::set_subcommand(app);

    let app_m = app.get_matches();
    match app_m.subcommand() {
        Some(("new", new_args)) => {new_command::exec_command(new_args)}
        _ => {}
    }
}