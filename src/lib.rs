use std::error::Error;
use std::process;

pub mod commands;

pub struct Config {
    issued_command: String,
    params: Vec<String>,
    options: Vec<String>
}

impl Config {
    pub fn build(args: Vec<String>) -> Config {
        if args.len() < 2 {
            eprintln!("Wrong amount of params. Use 'next-butler help' to see what you can do");
            process::exit(1);
        }

        let issued_command = args[1].clone();
        let mut params: Vec<String> = vec![];
        let mut options: Vec<String> = vec![];
        for arg in &args[2..] {
            if arg.contains("--") {
                options.push(arg.clone());
            } else {
                params.push(arg.clone());
            }
        }

        Config {
            issued_command,
            params,
            options
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    return match config.issued_command.as_str() {
        "help" => Ok(commands::help_command::show_help()),
        "new" => commands::new_command::create_file(config),
        _ => Err(Box::new(commands::CommandError::unknown_command(None)))
    };
}

#[cfg(test)]
mod tests {
    
}