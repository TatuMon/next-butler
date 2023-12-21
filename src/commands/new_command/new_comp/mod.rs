use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

use crate::helpers::file_helper::{self, get_name_or_err};

/// Sets the new component subcommand
pub fn set_subcommand(app: Command) -> Command {
    app.subcommand(
        Command::new("component")
            .about("Create a new component file, inside /components/")
            .arg(Arg::new("component_name").required(true).help(
                "The name of the component file. You can \
                              preppend the parents folder if needed (like \
                              /cards/<your_name>). Files are .jsx by default",
            ))
            .arg(
                Arg::new("tsx")
                    .help("Define if the file is a typescript one")
                    .long("tsx")
                    .required(false)
                    .action(ArgAction::SetTrue),
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
    let inputted_path = PathBuf::from(comp_args.get_one::<String>("component_name").unwrap());
    let is_tsx = comp_args.get_flag("tsx");
    let inputted_template = comp_args.get_one::<String>("template");

    let component_final_path = get_component_final_path(inputted_path, is_tsx)?;
    let component_name = get_name_or_err(&component_final_path)?;
    let component_content = get_component_content(component_name, inputted_template)?;

    file_helper::create(&component_final_path, component_content)?;

    Ok(())
}

fn get_component_final_path(inputted_path: PathBuf, is_tsx: bool) -> Result<PathBuf, String> {
    let component_path = add_component_ext(inputted_path, is_tsx)?;
    let component_path = add_component_prefix(component_path)?;

    Ok(component_path)
}

fn add_component_prefix(component_path: PathBuf) -> Result<PathBuf, String> {
    let component_relative_path = component_path
        .strip_prefix("/")
        .unwrap_or(component_path.as_path())
        .to_path_buf();

    let mut path_prefix = PathBuf::new();

    if file_helper::is_src_present()? {
        path_prefix.push("src/");
    }

    path_prefix.push("components/");

    let final_path = path_prefix.join(component_relative_path);

    Ok(final_path)
}

fn add_component_ext(mut component_path: PathBuf, is_tsx: bool) -> Result<PathBuf, String> {
    let ext_modified: bool = if is_tsx {
        component_path.set_extension("tsx")
    } else {
        component_path.set_extension("jsx")
    };

    if ext_modified {
        Ok(component_path)
    } else {
        Err(String::from("Couldn't set file extension"))
    }
}
