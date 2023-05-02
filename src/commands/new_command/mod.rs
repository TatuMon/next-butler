pub mod new_page;
pub mod new_comp;
pub mod new_style;

use clap::{Command, Arg, ArgMatches};

/// Settea el subcomando y los argumentos correspondientes
pub fn set_subcommand(app: Command) -> Command {
    let new_subcommand = Command::new("new")
        .about("Create a new page, component or stylesheet.")
        .arg(Arg::new("page")
            .required(true)
            .default_value("/api/coso")
            .help("The full name of the page")
            .long_help("The full name of the page. \
                        You can preppend the parents \
                        folder to specify it's location."))
        .arg(Arg::new("component")
            .conflicts_with_all(["page", "style"])
            .required(true)
            .help("The full name of the component")
            .long_help("The full name of the component. \
                        You can preppend the parents \
                        folder to specify it's location."))
        .arg(Arg::new("style")
            .conflicts_with_all(["page", "component"])
            .required(true)
            .help("The full name of the stylesheet")
            .long_help("The full name of the stylesheet. \
                        You can preppend the parents \
                        folder to specify it's location."));

    return app.subcommand(new_subcommand);
}

pub fn exec_command(new_args: &ArgMatches) {
    let page_name: &String = new_args.get_one("page").unwrap();

    println!("{}", *page_name);
}
