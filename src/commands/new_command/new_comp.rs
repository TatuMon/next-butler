use clap::{Command, Arg, ArgMatches};

pub fn set_subcommand(app: Command) -> Command {
    app.subcommand(Command::new("component")
                   .about("Create a new component file, inside /components/")
                   .arg(Arg::new("component_name")
                        .required(true)
                        .help("The name of the component file. You can \
                              preppend the parents folder if needed (like \
                              /cards/<your_name>)")))
}

pub fn exec_command(comp_args: &ArgMatches) -> Result<(), String> {
    Ok(())
}
