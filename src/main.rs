use next_butler::build_base_cmd;

fn main() {
    // Basic app setup
    let app = build_base_cmd();
    next_butler::run(app); 
}

#[cfg(test)]
pub mod tests;
