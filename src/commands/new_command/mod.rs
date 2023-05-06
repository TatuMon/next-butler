pub mod new_page;
pub mod new_comp;
pub mod new_style;

use clap::{Command, ArgMatches};

/// Settea el subcomando y los argumentos correspondientes
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

pub fn exec_command(new_args: &ArgMatches) {
    let subcmd = new_args.subcommand();
    let cmd_res = match subcmd {
        Some(("page", page_args)) => new_page::exec_command(page_args),
        Some(("component", comp_args)) => new_comp::exec_command(comp_args),
        Some(("style", style_args)) => new_style::exec_command(style_args),
        _ => Err(String::from("Unknown command"))
    };

    if let Err(err_msg) = cmd_res {
        eprintln!("{}", err_msg);
    }
}
