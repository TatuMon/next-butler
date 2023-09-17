pub mod new_page;
pub mod new_comp;
pub mod new_style;

use clap::{Command, ArgMatches};

/// Sets the subcommand and the corresponding arguments 
pub fn set_subcommand(app: Command) -> Command {
    // Set the subcommand 'new'
    let new_subcommand = Command::new("new")
        .about("Create a new page, component or stylesheet.");

    // Set the subcommand 'page' to 'new'
    let new_subcommand = new_page::set_subcommand(new_subcommand);
    // Set the subcommand 'component' to 'new'
    let new_subcommand = new_comp::set_subcommand(new_subcommand);
    // Set the subcommand 'style' to 'new'
    let new_subcommand = new_style::set_subcommand(new_subcommand);

    // Attaches the subcommand 'new' to the main command
    return app.subcommand(new_subcommand);
}

/// Executes the command
pub fn exec_command(cmd_args: &ArgMatches) {
    let subcmd = cmd_args.subcommand();
    let cmd_result = match subcmd {
        Some(("page", new_page_cmd_args)) => new_page::exec_command(new_page_cmd_args),
        Some(("component", new_comp_cmd_args)) => new_comp::exec_command(new_comp_cmd_args),
        Some(("style", new_style_cmd_args)) => new_style::exec_command(new_style_cmd_args),
        _ => Err(String::from("Unknown command"))
    };

    if let Err(err_msg) = cmd_result {
        eprintln!("{}", err_msg);
    }
}
