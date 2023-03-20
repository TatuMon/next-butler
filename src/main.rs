use std::env;

use next_butler::BaseConfig;

fn main() {
    // Read parameters
    let base_config = BaseConfig::build(env::args().collect());

    // Runs the app with the given base config
    if let Err(e) = next_butler::run(base_config) {
        eprintln!("{}", e);
    }
}
