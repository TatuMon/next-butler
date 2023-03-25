use std::env;

use next_butler::base_config::BaseConfig;

fn main() {
    // Parameters
    let params: Vec<String> = env::args().collect();

    // Build a base config based on the parameters
    let base_config = BaseConfig::build(params);

    // Runs the app with the given base config
    if let Err(e) = next_butler::run(base_config) {
        eprintln!("{}", e);
    }
}
