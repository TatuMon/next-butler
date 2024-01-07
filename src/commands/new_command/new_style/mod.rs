use clap::{Arg, ArgMatches, Command};

use crate::helpers::file_helper;

use self::final_new_style_config::FinalNewStyleConfig;

mod final_new_style_config;

/// Sets the new stylesheet subcommand
pub fn set_subcommand(app: Command) -> Command {
    app.subcommand(
        Command::new("style")
            .about("Create a new stylesheet, inside /styles/")
            .arg(Arg::new("style_name").required(true).help(
                "The name of the stylesheet. You can \
                              preppend the parents folder if needed (like \
                              /cards/<your_name>)",
            ))
            .arg(
                Arg::new("extension")
                    .help("Define the extension of the stylesheet")
                    .long("extension"),
            )
            .arg(
                Arg::new("folder")
                    .help("Define where to save the new stylesheet")
                    .long("folder"),
            )
            .arg(
                Arg::new("template")
                    .help("The name of your custom template")
                    .long("template"),
            ),
    )
}

/// Creates a new stylesheet based on the given arguments and the configuration file
pub fn exec_command(style_args: &ArgMatches) -> Result<(), String> {
    let style_config = FinalNewStyleConfig::new(style_args)?;

    file_helper::create(
        &style_config.style_final_path,
        style_config.template.content,
    )?;

    Ok(())
}
