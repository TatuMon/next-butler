use clap::Command;
use next_butler::constants::{CRATE_NAME, CRATE_VERSION};

fn main() {
    // Basic app setup
    let app = Command::new(CRATE_NAME)
        .about("You can configure next-butler creating a file named \
                nextbutler.json inside your root folder. Go to the README \
                for more information.")
        .version(CRATE_VERSION)
        .arg_required_else_help(true);

    next_butler::run(app); 
}
