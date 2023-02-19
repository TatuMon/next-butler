use std::env;

use next_butler::BaseConfig;

fn main() {
    let base_config = BaseConfig::build(env::args().collect());

    if let Err(e) = next_butler::run(base_config) {
        eprintln!("{}", e);
    }
}
