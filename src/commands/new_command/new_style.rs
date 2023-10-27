use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::helpers::{
    file_helper::{self, get_name_or_err},
    template_helper::get_stylesheet_content,
};

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
                Arg::new("scss")
                    .help("Define the extension of the stylesheet")
                    .long("scss")
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("sass")
                    .help("Define the extension of the stylesheet")
                    .long("sass")
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

/// Creates a new stylesheet based on the given arguments and the configuration file
pub fn exec_command(style_args: &ArgMatches) -> Result<(), String> {
    let inputted_path = PathBuf::from(style_args.get_one::<String>("style_name").unwrap());
    let scss_flag = style_args.get_flag("scss");
    let sass_flag = style_args.get_flag("sass");
    let template = style_args.get_one::<String>("template");

    let style_final_path = get_style_final_path(inputted_path, scss_flag, sass_flag)?;
    let style_name = get_name_or_err(&style_final_path)?;
    let style_content = get_stylesheet_content(style_name, template)?;

    file_helper::create(&style_final_path, style_content)?;

    Ok(())
}

fn get_style_final_path(
    inputted_path: PathBuf,
    is_scss: bool,
    is_sass: bool,
) -> Result<PathBuf, String> {
    let final_path = add_component_ext(inputted_path, is_scss, is_sass)?;
    let final_path = add_component_prefix(final_path)?;

    Ok(final_path)
}

fn add_component_ext(
    mut style_path: PathBuf,
    is_scss: bool,
    is_sass: bool,
) -> Result<PathBuf, String> {
    let ext_modified: bool;

    if is_scss {
        ext_modified = style_path.set_extension("scss");
    } else if is_sass {
        ext_modified = style_path.set_extension("sass");
    } else {
        ext_modified = style_path.set_extension("css");
    }

    if ext_modified {
        Ok(style_path)
    } else {
        Err(String::from("Couldn't set file extension"))
    }
}

fn add_component_prefix(style_path: PathBuf) -> Result<PathBuf, String> {
    let style_relative_path = style_path
        .strip_prefix("/")
        .unwrap_or(style_path.as_path())
        .to_path_buf();

    let mut path_prefix = PathBuf::new();

    if file_helper::is_src_present()? {
        path_prefix.push("src/");
    }

    path_prefix.push("styles/");

    let final_path = path_prefix.join(style_relative_path);

    Ok(final_path)
}
