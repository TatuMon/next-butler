use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::helpers::file_helper;

use self::final_new_comp_config::FinalNewCompConfig;

mod final_new_comp_config;

/// Sets the new component subcommand
pub fn set_subcommand(app: Command) -> Command {
    app.subcommand(
        Command::new("component")
            .about("Create a new component file, inside /components/")
            .arg(Arg::new("component_path").required(true).help(
                "The name of the component file. You can \
                              preppend the parents folder if needed (like \
                              /cards/<your_name>). Files are .jsx by default",
            ))
            .arg(
                Arg::new("js")
                .help(
                    "Define if the file should have the .js extension\
                    (.jsx is the default for pages)",
                    )
                .long("js")
                .action(ArgAction::SetTrue),
                )
            .arg(
                Arg::new("ts")
                    .help("Define if the file is a typescript one")
                    .long("ts")
                    .action(ArgAction::SetTrue)
                    .conflicts_with("js"),
            )
            .arg(
                Arg::new("jsx")
                    .help("Define if the file should have the .jsx extension")
                    .long("jsx")
                    .conflicts_with("js")
                    .conflicts_with("ts")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("tsx")
                    .help("Define if the file should have the .tsx extension")
                    .long("tsx")
                    .conflicts_with("js")
                    .conflicts_with("ts")
                    .conflicts_with("jsx")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("folder")
                    .help("Define the base folder of the component")
                    .long("folder")
            )
            .arg(
                Arg::new("template")
                    .help("The name of your custom template")
                    .long("template"),
            ),
    )
}

/// Creates a new component based on the given arguments and the configuration file
pub fn exec_command(comp_args: &ArgMatches) -> Result<(), String> {
    let component_config = FinalNewCompConfig::new(comp_args)?;

    file_helper::create(
        &component_config.comp_final_path,
        component_config.template.content,
    )?;

    Ok(())
}
