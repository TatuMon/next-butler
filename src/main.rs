fn main() {
    // Basic app setup
    let base_cmd = next_butler::build_base_cmd();
    next_butler::run(base_cmd);
}

#[cfg(test)]
pub mod tests;
