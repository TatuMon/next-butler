use std::env;

use next_butler::Config;

fn main() {
    let config = Config::build(env::args().collect());

    if let Err(e) = next_butler::run(config) {
        eprintln!("{}", e);
    }
}
