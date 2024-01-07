use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::helpers::file_helper;

use self::final_new_page_config::FinalNewPageConfig;

pub mod final_new_page_config;

/// Sets the new page subcommand
pub fn set_subcommand(app: Command) -> Command {
    app.subcommand(
        Command::new("page")
            .about("Create a new page file, inside /pages/")
            .arg(
                Arg::new("page_path")
                    .required(true)
                    .help("The path of the page file."),
            )
            .arg(
                Arg::new("ts")
                    .help("Define if the file is a typescript one")
                    .conflicts_with("js")
                    .long("ts")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
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
                Arg::new("jsx")
                    .help("Define if the file should have the .jsx extension")
                    .long("jsx")
                    .conflicts_with("tsx")
                    .conflicts_with("ts")
                    .conflicts_with("js")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("tsx")
                    .help("Define if the file should have the .tsx extension")
                    .long("tsx")
                    .conflicts_with("jsx")
                    .conflicts_with("ts")
                    .conflicts_with("js")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("template")
                    .help("The name of your custom template")
                    .long("template"),
            )
            .arg(
                Arg::new("page-router")
                    .help("Create the page based on the page router")
                    .conflicts_with("app-router")
                    .long("page-router")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("app-router")
                    .help("Create the page based on the app router")
                    .long("app-router")
                    .action(ArgAction::SetTrue),
            ),
    )
}

/// Creates a new page based on the given arguments and the configuration file
pub fn exec_command(cmd_args: &ArgMatches) -> Result<(), String> {
    let page_config = FinalNewPageConfig::new(cmd_args)?;

    file_helper::create(&page_config.page_final_path, page_config.template.content)?;
    Ok(())
}
