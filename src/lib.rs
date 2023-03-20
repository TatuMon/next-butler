use std::error::Error;
use std::path::Path;
use std::{process, fs, str};

pub mod commands;
pub mod helpers;
pub mod json_config;

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
    if !is_next_project() {
        return Err(Box::new(commands::CommandError::wrong_location()));
    }

    return match config.issued_command.as_str() {
        "help" => Ok(commands::help_command::show_help()),
        "new" => commands::new_command::create_file(config),
        _ => Err(Box::new(commands::CommandError::unknown_command(None)))
    };
}

fn is_next_project() -> bool {
    // package.json exists?
    let package_json_path = Path::new("./package.json");
    if !package_json_path.exists() { return false; } else {};

    // is package.json readable?
    let package_vec_u8: Vec<u8>;
    match fs::read(package_json_path) {
        Ok(vector) => package_vec_u8 = vector,
        Err(_) => return false
    }

    // is packahe.json an object?
    let package_json = json::parse(str::from_utf8(&package_vec_u8).unwrap_or("")).unwrap_or(json::from(""));

    if !package_json.is_object() {
        return false;
    };

    // has dependencies or devDependencies?
    if !package_json.has_key("dependencies") && !package_json.has_key("devDependencies") {
        return false;
    };

    // does dependencies or devDependencies have "next"?
    if (!package_json["dependencies"].is_object() || !package_json["dependencies"].has_key("next"))
        &&
        (!package_json["devDependencies"].is_object() || !package_json["devDependencies"].has_key("next")){
        return false;
    };

    // is "next" a string?
    if package_json["dependencies"]["next"].is_string() || package_json["devDependencies"]["next"].is_string(){
        return true;
    };

    return false;

}

#[cfg(test)]
mod tests {
    use crate::{helpers::str_helper};

    #[test]
    fn to_pascal_wrong_start() {
        let to_parse = "amazing{s-";
        let parsed = str_helper::str_to_pascal_case(to_parse);
        
        assert!(parsed.is_err());
    }
}