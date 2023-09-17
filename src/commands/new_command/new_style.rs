use clap::{Command, Arg, ArgMatches};

/// Sets the new stylesheet subcommand
pub fn set_subcommand(app: Command) -> Command {
    app.subcommand(Command::new("style")
                   .about("Create a new stylesheet, inside /styles/")
                   .arg(Arg::new("style_name")
                        .required(true)
                        .help("The name of the stylesheet. You can \
                              preppend the parents folder if needed (like \
                              /cards/<your_name>)")))
}

/// Creates a new stylesheet based on the given arguments and the configuration file
pub fn exec_command(style_args: &ArgMatches) -> Result<(), String> {
    Ok(())
}
