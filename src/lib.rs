use std::error::Error;
use std::process;

pub mod commands;
pub mod helpers;

pub struct BaseConfig {
    issued_command: String,
    params: Vec<String>,
    // options: Vec<String> Not in use yet
}

impl BaseConfig {
    pub fn build(args: Vec<String>) -> BaseConfig {
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

        BaseConfig {
            issued_command,
            params,
            // options
        }
    }
}

pub fn run(config: BaseConfig) -> Result<(), Box<dyn Error>> {
    return match config.issued_command.as_str() {
        "help" => Ok(commands::help_command::show_help()),
        "new" => commands::new_command::create_file(config),
        _ => Err(Box::new(commands::CommandError::unknown_command(None)))
    };
}

#[cfg(test)]
mod tests {
    use crate::helpers::str_helper;

    #[test]
    fn to_pascal_wrong_start() {
        let to_parse = "amazing{s-";
        let parsed = str_helper::str_to_pascal_case(to_parse);
        
        assert!(parsed.is_err());
    }
}